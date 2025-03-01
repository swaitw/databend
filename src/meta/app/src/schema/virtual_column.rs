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

use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use chrono::DateTime;
use chrono::Utc;
use databend_common_expression::TableDataType;
use databend_common_meta_types::MetaId;

use super::CreateOption;
use crate::schema::virtual_column_ident::VirtualColumnIdent;
use crate::tenant::Tenant;
use crate::tenant::ToTenant;

// The virtual field column definition of Variant type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VirtualField {
    // Expression to extracts the internal virtual field of the variant value.
    // for example:
    // `data['key']`, `data[0]`, `data['key1']['key2']`, ..
    pub expr: String,
    // The data type of internal virtual field.
    // If all the rows of a virtual field has same type,
    // the virtual field can cast to the type.
    pub data_type: TableDataType,
    // Optional alias name.
    pub alias_name: Option<String>,
}

impl Display for VirtualField {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Some(alias_name) = &self.alias_name {
            write!(
                f,
                "{}::{} AS {}",
                self.expr,
                self.data_type.remove_nullable(),
                alias_name
            )
        } else {
            write!(f, "{}::{}", self.expr, self.data_type.remove_nullable())
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VirtualColumnMeta {
    pub table_id: MetaId,

    // The internal virtual field columns of Variant type.
    // For example, the data column has the following values:
    // `{"id":1,"name":"tom","metas":{"key1":"val1","key2":"val2"}}`
    // `{"id":2,"name":"alice","metas":{"key1":"val3","key2":"val4"}}`
    // ...
    // We can generate virtual columns as follows:
    // `data['id']`, `data['name']`, `data['metas']['key1']`, `data['metas']['key2']`
    pub virtual_columns: Vec<VirtualField>,
    pub created_on: DateTime<Utc>,
    pub updated_on: Option<DateTime<Utc>>,
    // Whether the virtual columns are auto-generated,
    // true for auto-generated, false for user-defined.
    pub auto_generated: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateVirtualColumnReq {
    pub create_option: CreateOption,
    pub name_ident: VirtualColumnIdent,
    pub virtual_columns: Vec<VirtualField>,
    pub auto_generated: bool,
}

impl Display for CreateVirtualColumnReq {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "create_virtual_column ({:?}) for {}",
            self.virtual_columns,
            self.name_ident.display()
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UpdateVirtualColumnReq {
    pub if_exists: bool,
    pub name_ident: VirtualColumnIdent,
    pub virtual_columns: Vec<VirtualField>,
    pub auto_generated: bool,
}

impl Display for UpdateVirtualColumnReq {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "update_virtual_column ({:?}) for {}",
            self.virtual_columns,
            self.name_ident.display()
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DropVirtualColumnReq {
    pub if_exists: bool,
    pub name_ident: VirtualColumnIdent,
}

impl Display for DropVirtualColumnReq {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "drop_virtual_column for {}", self.name_ident.display())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListVirtualColumnsReq {
    pub tenant: Tenant,
    pub table_id: Option<MetaId>,
}

impl ListVirtualColumnsReq {
    pub fn new(tenant: impl ToTenant, table_id: Option<MetaId>) -> ListVirtualColumnsReq {
        ListVirtualColumnsReq {
            tenant: tenant.to_tenant(),
            table_id,
        }
    }
}
