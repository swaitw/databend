// Copyright 2021 Datafuse Labs
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt::Display;
use std::fmt::Formatter;

use serde::Deserialize;
use serde::Serialize;

use crate::protobuf as pb;
use crate::seq_value::SeqV;
use crate::ConflictSeq;

/// Describes what `seq` an operation must match to take effect.
/// Every value written to meta data has a unique `seq` bound.
/// Any conditioned or non-conditioned write operation can be done through the corresponding MatchSeq.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, deepsize::DeepSizeOf)]
pub enum MatchSeq {
    // TODO(xp): remove Any, it is equivalent to GE(0)
    /// Any value is acceptable, i.e. does not check seq at all.
    Any,

    /// To match an exact value of seq.
    /// E.g., CAS updates the exact version of some value,
    /// and put-if-absent adds a value only when seq is 0.
    Exact(u64),

    /// To match a seq that is greater-or-equal some value.
    /// E.g., GE(1) perform an update on any existent value.
    GE(u64),
}

impl Display for MatchSeq {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            MatchSeq::Any => {
                write!(f, "is any value")
            }
            MatchSeq::Exact(s) => {
                write!(f, "== {}", s)
            }
            MatchSeq::GE(s) => {
                write!(f, ">= {}", s)
            }
        }
    }
}

pub trait MatchSeqExt<T> {
    /// Match against a some value containing seq by checking if the seq satisfies the condition.
    fn match_seq(&self, sv: T) -> Result<(), ConflictSeq>;
}

impl<U> MatchSeqExt<&Option<SeqV<U>>> for MatchSeq {
    fn match_seq(&self, sv: &Option<SeqV<U>>) -> Result<(), ConflictSeq> {
        let seq = sv.as_ref().map_or(0, |sv| sv.seq);
        self.match_seq(seq)
    }
}

impl MatchSeqExt<&Option<pb::SeqV>> for MatchSeq {
    fn match_seq(&self, sv: &Option<pb::SeqV>) -> Result<(), ConflictSeq> {
        let seq = sv.as_ref().map_or(0, |sv| sv.seq);
        self.match_seq(seq)
    }
}

impl<U> MatchSeqExt<&SeqV<U>> for MatchSeq {
    fn match_seq(&self, sv: &SeqV<U>) -> Result<(), ConflictSeq> {
        let seq = sv.seq;
        self.match_seq(seq)
    }
}

impl MatchSeqExt<u64> for MatchSeq {
    fn match_seq(&self, seq: u64) -> Result<(), ConflictSeq> {
        match self {
            MatchSeq::Any => Ok(()),
            MatchSeq::Exact(s) if seq == *s => Ok(()),
            MatchSeq::GE(s) if seq >= *s => Ok(()),
            _ => Err(ConflictSeq::NotMatch {
                want: *self,
                got: seq,
            }),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::seq_value::SeqV;
    use crate::ConflictSeq;
    use crate::MatchSeq;
    use crate::MatchSeqExt;

    #[derive(serde::Serialize)]
    struct Foo {
        f: MatchSeq,
    }

    #[test]
    fn test_match_seq_serde() -> anyhow::Result<()> {
        //

        let t = Foo { f: MatchSeq::Any };
        let s = serde_json::to_string(&t)?;
        println!("{s}");

        Ok(())
    }

    #[test]
    fn test_match_seq_match_seq_value() -> anyhow::Result<()> {
        assert_eq!(MatchSeq::GE(0).match_seq(&Some(SeqV::new(0, 1))), Ok(()));
        assert_eq!(MatchSeq::GE(0).match_seq(&Some(SeqV::new(1, 1))), Ok(()));

        //

        assert_eq!(
            MatchSeq::Exact(3).match_seq(&None::<SeqV>),
            Err(ConflictSeq::NotMatch {
                want: MatchSeq::Exact(3),
                got: 0
            })
        );
        assert_eq!(
            MatchSeq::Exact(3).match_seq(&Some(SeqV::new(0, 1))),
            Err(ConflictSeq::NotMatch {
                want: MatchSeq::Exact(3),
                got: 0
            })
        );
        assert_eq!(
            MatchSeq::Exact(3).match_seq(&Some(SeqV::new(2, 1))),
            Err(ConflictSeq::NotMatch {
                want: MatchSeq::Exact(3),
                got: 2
            })
        );
        assert_eq!(MatchSeq::Exact(3).match_seq(&Some(SeqV::new(3, 1))), Ok(()));
        assert_eq!(
            MatchSeq::Exact(3).match_seq(&Some(SeqV::new(4, 1))),
            Err(ConflictSeq::NotMatch {
                want: MatchSeq::Exact(3),
                got: 4
            })
        );

        //

        assert_eq!(
            MatchSeq::GE(3).match_seq(&None::<SeqV>),
            Err(ConflictSeq::NotMatch {
                want: MatchSeq::GE(3),
                got: 0
            })
        );
        assert_eq!(
            MatchSeq::GE(3).match_seq(&Some(SeqV::new(0, 1))),
            Err(ConflictSeq::NotMatch {
                want: MatchSeq::GE(3),
                got: 0
            })
        );
        assert_eq!(
            MatchSeq::GE(3).match_seq(&Some(SeqV::new(2, 1))),
            Err(ConflictSeq::NotMatch {
                want: MatchSeq::GE(3),
                got: 2
            })
        );
        assert_eq!(MatchSeq::GE(3).match_seq(&Some(SeqV::new(3, 1))), Ok(()));
        assert_eq!(MatchSeq::GE(3).match_seq(&Some(SeqV::new(4, 1))), Ok(()));

        Ok(())
    }

    #[test]
    fn test_match_seq_display() -> anyhow::Result<()> {
        assert_eq!("== 3", MatchSeq::Exact(3).to_string());
        assert_eq!(">= 3", MatchSeq::GE(3).to_string());

        Ok(())
    }
}
