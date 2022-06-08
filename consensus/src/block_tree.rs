use std::{cmp, collections::HashMap, hash::Hash};

use crate::{hash, ledger::Ledger, BlockRound, F};

#[derive(Clone, Hash)]
pub struct VoteInfo {
    // Id of block
    pub id: BlockId,
    // round of block
    pub round: BlockRound,
    // Id of parent
    pub parent_id: BlockId,
    // round of parent
    pub parent_round: BlockRound,
    // Speculated execution state
    pub exec_state_id: Option<()>,
}

// speculated new committed state to vote directly on
#[derive(Clone, Hash)]
pub struct LedgerCommitInfo {
    // ⊥ if no commit happens when this vote is aggregated to QC
    pub commit_state_id: Option<()>,
    // Hash of VoteMsg.vote info
    pub vote_info_hash: u64,
}

pub struct VoteMsg {
    // A VoteInfo record
    vote_info: VoteInfo,
    // Speculated ledger info
    ledger_commit_info: LedgerCommitInfo,
    // QC to synchronize on committed blocks
    high_commit_qc: QuorumCertificate,
    // Added automatically when constructed
    sender: (),
    // Signed automatically when constructed
    signature: (),
}

impl VoteMsg {
    pub fn new(vote_info: VoteInfo, ledger_commit_info: LedgerCommitInfo, high_commit_qc: QuorumCertificate, author: ()) -> Self {
        Self {
            vote_info,
            ledger_commit_info,
            high_commit_qc,
            sender: (),
            signature: (),
        }
    }
}

// QC is a VoteMsg with multiple signatures
#[derive(Clone)]
pub struct QuorumCertificate {
    pub vote_info: VoteInfo,
    ledger_commit_info: LedgerCommitInfo,
    // A quorum of signatures
    pub signatures: Vec<()>,
    // The validator that produced the qc
    author: (),
    author_signature: (),
}

impl PartialEq for QuorumCertificate {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Eq for QuorumCertificate {}

impl Ord for QuorumCertificate {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        todo!()
    }
}

impl PartialOrd for QuorumCertificate {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// FIXME: integrate with the snarkVM BlockHash OR height
pub type BlockId = u64;

// FIXME: integrate with the snarkVM Block
#[derive(Clone)]
pub struct Block {
    // The author of the block, may not be the same as qc.author after view-change
    pub author: (),
    // The round that generated this proposal
    pub round: BlockRound,
    // Proposed transaction(s)
    payload: Vec<()>,
    // QC for parent block
    pub qc: QuorumCertificate,
    // A unique digest of author, round, payload, qc.vote info.id and qc.signatures
    pub id: BlockId,
}

pub struct BlockTree {
    // tree of blocks pending commitment
    pending_block_tree: HashMap<BlockId, Block>,
    // collected votes per block indexed by their LedgerInfo hash
    pending_votes: HashMap<u64, Vec<VoteMsg>>,
    // highest known QC
    pub high_qc: QuorumCertificate,
    // highest QC that serves as a commit certificate
    pub high_commit_qc: QuorumCertificate,
}

impl BlockTree {
    pub fn process_qc(&mut self, qc: QuorumCertificate, ledger: &mut Ledger) {
        if qc.ledger_commit_info.commit_state_id.is_none() {
            ledger.commit(qc.vote_info.parent_id.clone());
            self.pending_block_tree.remove(&qc.vote_info.parent_id);
            if qc > self.high_commit_qc {
                self.high_commit_qc = qc.clone();
            }
        }

        if qc > self.high_qc {
            self.high_qc = qc;
        }
    }

    pub fn execute_and_insert(&mut self, b: Block, ledger: &mut Ledger) {
        ledger.speculate(b.qc.vote_info.parent_id.clone(), b.id.clone(), b.payload.clone());

        self.pending_block_tree.insert(b.id.clone(), b);
    }

    pub fn process_vote(&mut self, v: VoteMsg, ledger: &mut Ledger) -> Option<QuorumCertificate> {
        self.process_qc(v.high_commit_qc, ledger);

        let vote_idx = hash(&v.ledger_commit_info);
        let mut pending_votes = self.pending_votes.entry(vote_idx).or_default();

        /*

        FIXME: so does this collection contain votes or signatures?
               are they the same thing here?

        pending_votes[vote_idx] ← pending_votes[vote_idx] ∪ v.signature

        pending_votes.push(v.signature);

        */

        if pending_votes.len() == 2 * F + 1 {
            /*

            FIXME: this QC is different than the one defined in page 10
                   is it just a broad way of creating a QC from VoteInfo and LedgerCommitInfo?

            QC〈
                vote_info ← v.vote_info,
                state_id ← v.state_id,
                votes ← self.pending_votes[vote idx]
            〉

            return Some(QuorumCertificate {

            })

            */
        }

        None
    }

    pub fn generate_block(&self, txns: Vec<()>, current_round: BlockRound) -> Block {
        /*

        TODO: roll the values below into the desired hash

        let id = hash(&[AUTHOR, &current_round, &txns, &self.high_qc.vote_info.id, &self.high_qc.signatures]);

        */
        let id = 0;

        Block {
            author: (), // TODO: it's the own validator ID
            round: current_round,
            payload: txns,
            qc: self.high_qc.clone(),
            id,
        }
    }
}