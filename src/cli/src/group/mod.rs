use std::sync::Arc;
use structopt::StructOpt;

mod create;
mod delete;
mod list;

use fluvio::Fluvio;
use crate::Result;
use crate::common::output::Terminal;
use crate::common::COMMAND_TEMPLATE;
use crate::group::create::CreateManagedSpuGroupOpt;
use crate::group::delete::DeleteManagedSpuGroupOpt;
use crate::group::list::ListManagedSpuGroupsOpt;

#[derive(Debug, StructOpt)]
pub enum SpuGroupCmd {
    /// Create a new managed SPU Group
    #[structopt(
        name = "create",
        template = COMMAND_TEMPLATE,
    )]
    Create(CreateManagedSpuGroupOpt),

    /// Delete a managed SPU Group
    #[structopt(
        name = "delete",
        template = COMMAND_TEMPLATE,
    )]
    Delete(DeleteManagedSpuGroupOpt),

    /// List all managed SPUs
    #[structopt(
        name = "list",
        template = COMMAND_TEMPLATE,
    )]
    List(ListManagedSpuGroupsOpt),
}

impl SpuGroupCmd {
    pub async fn process<O: Terminal>(self, out: Arc<O>, fluvio: &Fluvio) -> Result<()> {
        match self {
            Self::Create(create) => {
                create.process(fluvio).await?;
            }
            Self::Delete(delete) => {
                delete.process(fluvio).await?;
            }
            Self::List(list) => {
                list.process(out, fluvio).await?;
            }
        }
        Ok(())
    }
}
