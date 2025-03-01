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

#[inline]
pub fn position1<const POSITIVE: bool, const C1: u8>(buf: &[u8]) -> usize {
    position16::<POSITIVE, C1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0>(buf)
}

#[inline]
pub fn position2<const POSITIVE: bool, const C1: u8, const C2: u8>(buf: &[u8]) -> usize {
    position16::<POSITIVE, C1, C2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0>(buf)
}

#[inline]
pub fn position3<const POSITIVE: bool, const C1: u8, const C2: u8, const C3: u8>(
    buf: &[u8],
) -> usize {
    position16::<POSITIVE, C1, C2, C3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0>(buf)
}

#[inline]
pub fn position4<const POSITIVE: bool, const C1: u8, const C2: u8, const C3: u8, const C4: u8>(
    buf: &[u8],
) -> usize {
    position16::<POSITIVE, C1, C2, C3, C4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0>(buf)
}

#[inline]
pub fn position5<
    const POSITIVE: bool,
    const C1: u8,
    const C2: u8,
    const C3: u8,
    const C4: u8,
    const C5: u8,
>(
    buf: &[u8],
) -> usize {
    position16::<POSITIVE, C1, C2, C3, C4, C5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0>(buf)
}

#[inline]
pub fn position6<
    const POSITIVE: bool,
    const C1: u8,
    const C2: u8,
    const C3: u8,
    const C4: u8,
    const C5: u8,
    const C6: u8,
>(
    buf: &[u8],
) -> usize {
    position16::<POSITIVE, C1, C2, C3, C4, C5, C6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0>(buf)
}

#[inline]
pub fn position7<
    const POSITIVE: bool,
    const C1: u8,
    const C2: u8,
    const C3: u8,
    const C4: u8,
    const C5: u8,
    const C6: u8,
    const C7: u8,
>(
    buf: &[u8],
) -> usize {
    position16::<POSITIVE, C1, C2, C3, C4, C5, C6, C7, 0, 0, 0, 0, 0, 0, 0, 0, 0>(buf)
}

#[inline]
pub fn position8<
    const POSITIVE: bool,
    const C1: u8,
    const C2: u8,
    const C3: u8,
    const C4: u8,
    const C5: u8,
    const C6: u8,
    const C7: u8,
    const C8: u8,
>(
    buf: &[u8],
) -> usize {
    position16::<POSITIVE, C1, C2, C3, C4, C5, C6, C7, C8, 0, 0, 0, 0, 0, 0, 0, 0>(buf)
}

#[inline]
pub fn position9<
    const POSITIVE: bool,
    const C1: u8,
    const C2: u8,
    const C3: u8,
    const C4: u8,
    const C5: u8,
    const C6: u8,
    const C7: u8,
    const C8: u8,
    const C9: u8,
>(
    buf: &[u8],
) -> usize {
    position16::<POSITIVE, C1, C2, C3, C4, C5, C6, C7, C8, C9, 0, 0, 0, 0, 0, 0, 0>(buf)
}

#[inline]
pub fn position10<
    const POSITIVE: bool,
    const C1: u8,
    const C2: u8,
    const C3: u8,
    const C4: u8,
    const C5: u8,
    const C6: u8,
    const C7: u8,
    const C8: u8,
    const C9: u8,
    const C10: u8,
>(
    buf: &[u8],
) -> usize {
    position16::<POSITIVE, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, 0, 0, 0, 0, 0, 0>(buf)
}

#[inline]
pub fn position11<
    const POSITIVE: bool,
    const C1: u8,
    const C2: u8,
    const C3: u8,
    const C4: u8,
    const C5: u8,
    const C6: u8,
    const C7: u8,
    const C8: u8,
    const C9: u8,
    const C10: u8,
    const C11: u8,
>(
    buf: &[u8],
) -> usize {
    position16::<POSITIVE, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, 0, 0, 0, 0, 0>(buf)
}

#[inline]
pub fn position12<
    const POSITIVE: bool,
    const C1: u8,
    const C2: u8,
    const C3: u8,
    const C4: u8,
    const C5: u8,
    const C6: u8,
    const C7: u8,
    const C8: u8,
    const C9: u8,
    const C10: u8,
    const C11: u8,
    const C12: u8,
>(
    buf: &[u8],
) -> usize {
    position16::<POSITIVE, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, 0, 0, 0, 0>(buf)
}

#[inline]
pub fn position13<
    const POSITIVE: bool,
    const C1: u8,
    const C2: u8,
    const C3: u8,
    const C4: u8,
    const C5: u8,
    const C6: u8,
    const C7: u8,
    const C8: u8,
    const C9: u8,
    const C10: u8,
    const C11: u8,
    const C12: u8,
    const C13: u8,
>(
    buf: &[u8],
) -> usize {
    position16::<POSITIVE, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, 0, 0, 0>(buf)
}

#[inline]
pub fn position14<
    const POSITIVE: bool,
    const C1: u8,
    const C2: u8,
    const C3: u8,
    const C4: u8,
    const C5: u8,
    const C6: u8,
    const C7: u8,
    const C8: u8,
    const C9: u8,
    const C10: u8,
    const C11: u8,
    const C12: u8,
    const C13: u8,
    const C14: u8,
>(
    buf: &[u8],
) -> usize {
    position16::<POSITIVE, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, 0, 0>(buf)
}

#[inline]
pub fn position15<
    const POSITIVE: bool,
    const C1: u8,
    const C2: u8,
    const C3: u8,
    const C4: u8,
    const C5: u8,
    const C6: u8,
    const C7: u8,
    const C8: u8,
    const C9: u8,
    const C10: u8,
    const C11: u8,
    const C12: u8,
    const C13: u8,
    const C14: u8,
    const C15: u8,
>(
    buf: &[u8],
) -> usize {
    position16::<POSITIVE, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, 0>(buf)
}

#[inline(always)]
#[allow(unreachable_code)]
pub fn position16<
    const POSITIVE: bool,
    const C1: u8,
    const C2: u8,
    const C3: u8,
    const C4: u8,
    const C5: u8,
    const C6: u8,
    const C7: u8,
    const C8: u8,
    const C9: u8,
    const C10: u8,
    const C11: u8,
    const C12: u8,
    const C13: u8,
    const C14: u8,
    const C15: u8,
    const C16: u8,
>(
    buf: &[u8],
) -> usize {
    #[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
    return position_sse42::<
        POSITIVE,
        C1,
        C2,
        C3,
        C4,
        C5,
        C6,
        C7,
        C8,
        C9,
        C10,
        C11,
        C12,
        C13,
        C14,
        C15,
        C16,
    >(buf);

    position16_from_index::<
        POSITIVE,
        C1,
        C2,
        C3,
        C4,
        C5,
        C6,
        C7,
        C8,
        C9,
        C10,
        C11,
        C12,
        C13,
        C14,
        C15,
        C16,
    >(buf, 0)
}

#[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
#[inline(always)]
fn position_sse42<
    const POSITIVE: bool,
    const C1: u8,
    const C2: u8,
    const C3: u8,
    const C4: u8,
    const C5: u8,
    const C6: u8,
    const C7: u8,
    const C8: u8,
    const C9: u8,
    const C10: u8,
    const C11: u8,
    const C12: u8,
    const C13: u8,
    const C14: u8,
    const C15: u8,
    const C16: u8,
>(
    buf: &[u8],
) -> usize {
    unsafe {
        use std::arch::x86_64::*;
        let chars_set = _mm_setr_epi8(
            C1 as i8, C2 as i8, C3 as i8, C4 as i8, C5 as i8, C6 as i8, C7 as i8, C8 as i8,
            C9 as i8, C10 as i8, C11 as i8, C12 as i8, C13 as i8, C14 as i8, C15 as i8, C16 as i8,
        );

        let chars_count = if C16 != 0 {
            16
        } else if C15 != 0 {
            15
        } else if C14 != 0 {
            14
        } else if C13 != 0 {
            13
        } else if C12 != 0 {
            12
        } else if C11 != 0 {
            11
        } else if C10 != 0 {
            10
        } else if C9 != 0 {
            9
        } else if C8 != 0 {
            8
        } else if C7 != 0 {
            7
        } else if C6 != 0 {
            6
        } else if C5 != 0 {
            5
        } else if C4 != 0 {
            4
        } else if C3 != 0 {
            3
        } else if C2 != 0 {
            2
        } else {
            1
        };

        let mut index = 0;

        while index + 15 < buf.len() {
            let bytes = _mm_loadu_si128(buf.as_ptr().add(index) as *const _);

            if POSITIVE {
                if _mm_cmpestrc::<0>(chars_set, chars_count, bytes, 16) > 0 {
                    return index + _mm_cmpestri::<0>(chars_set, chars_count, bytes, 16) as usize;
                }
            } else if _mm_cmpestrc::<_SIDD_NEGATIVE_POLARITY>(chars_set, chars_count, bytes, 16) > 0
            {
                return index
                    + _mm_cmpestri::<_SIDD_NEGATIVE_POLARITY>(chars_set, chars_count, bytes, 16)
                        as usize;
            }

            index += 16;
        }

        position16_from_index::<
            POSITIVE,
            C1,
            C2,
            C3,
            C4,
            C5,
            C6,
            C7,
            C8,
            C9,
            C10,
            C11,
            C12,
            C13,
            C14,
            C15,
            C16,
        >(buf, index)
    }
}

#[inline(always)]
fn position16_from_index<
    const POSITIVE: bool,
    const C1: u8,
    const C2: u8,
    const C3: u8,
    const C4: u8,
    const C5: u8,
    const C6: u8,
    const C7: u8,
    const C8: u8,
    const C9: u8,
    const C10: u8,
    const C11: u8,
    const C12: u8,
    const C13: u8,
    const C14: u8,
    const C15: u8,
    const C16: u8,
>(
    buf: &[u8],
    begin: usize,
) -> usize {
    let mut index = begin;
    while index < buf.len() {
        if POSITIVE
            == (buf[index] == C1
                || (C2 != 0 && buf[index] == C2)
                || (C3 != 0 && buf[index] == C3)
                || (C4 != 0 && buf[index] == C4)
                || (C5 != 0 && buf[index] == C5)
                || (C6 != 0 && buf[index] == C6)
                || (C7 != 0 && buf[index] == C7)
                || (C8 != 0 && buf[index] == C8)
                || (C9 != 0 && buf[index] == C9)
                || (C10 != 0 && buf[index] == C10)
                || (C11 != 0 && buf[index] == C11)
                || (C12 != 0 && buf[index] == C12)
                || (C13 != 0 && buf[index] == C13)
                || (C14 != 0 && buf[index] == C14)
                || (C15 != 0 && buf[index] == C15)
                || (C16 != 0 && buf[index] == C16))
        {
            return index;
        }

        index += 1;
    }

    index
}
