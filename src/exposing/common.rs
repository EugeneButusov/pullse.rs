use super::PullseLedger;

pub trait PullseExposer {
    fn consume(&self, ledger: &PullseLedger);
}
