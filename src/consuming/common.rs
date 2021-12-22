use crate::PullseLedger;

pub trait PullseConsumer {
    fn consume(&self, ledger: &PullseLedger);
}
