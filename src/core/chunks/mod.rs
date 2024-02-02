use self::{
    dummy::DummyChunk,
    gen8::ChunkGEN8,
    optn::ChunkOPTN,
    lang::ChunkLANG,
    extn::ChunkEXTN,
    sond::ChunkSOND,
    agrp::ChunkAGRP,
    sprt::ChunkSPRT,
    bgnd::ChunkBGND,
    path::ChunkPATH,
    scpt::ChunkSCPT,
    glob::ChunkGLOB,
    shdr::ChunkSHDR,
    font::ChunkFONT,
    tmln::ChunkTMLN,
    objt::ChunkOBJT,
    feds::ChunkFEDS, acrv::ChunkACRV, seqn::ChunkSEQN,
};

pub mod dummy;

pub mod gen8;
pub mod optn;
pub mod lang;
pub mod extn;
pub mod sond;
pub mod agrp;
pub mod sprt;
pub mod bgnd;
pub mod path;
pub mod scpt;
pub mod glob;
pub mod shdr;
pub mod font;
pub mod tmln;
pub mod objt;
pub mod feds;
pub mod acrv;
pub mod seqn;

#[derive(Clone)]
pub enum ChunkOutput {
    DummyChunk(DummyChunk),

    ChunkGen8(ChunkGEN8),
    ChunkOptn(ChunkOPTN),
    ChunkLang(ChunkLANG),
    ChunkExtn(ChunkEXTN),
    ChunkSond(ChunkSOND),
    ChunkAgrp(ChunkAGRP),
    ChunkSprt(ChunkSPRT),
    ChunkBgnd(ChunkBGND),
    ChunkPath(ChunkPATH),
    ChunkScpt(ChunkSCPT),
    ChunkGlob(ChunkGLOB),
    ChunkShdr(ChunkSHDR),
    ChunkFont(ChunkFONT),
    ChunkTmln(ChunkTMLN),
    ChunkObjt(ChunkOBJT),
    ChunkFeds(ChunkFEDS),
    ChunkAcrv(ChunkACRV),
    ChunkSeqn(ChunkSEQN),
}

impl From<DummyChunk> for ChunkOutput {
    fn from(value: DummyChunk) -> Self {
        Self::DummyChunk(value)
    }
}

impl From<ChunkGEN8> for ChunkOutput {
    fn from(value: ChunkGEN8) -> Self {
        Self::ChunkGen8(value)
    }
}

impl From<ChunkOPTN> for ChunkOutput {
    fn from(value: ChunkOPTN) -> Self {
        Self::ChunkOptn(value)
    }
}

impl From<ChunkLANG> for ChunkOutput {
    fn from(value: ChunkLANG) -> Self {
        Self::ChunkLang(value)
    }
}

impl From<ChunkEXTN> for ChunkOutput {
    fn from(value: ChunkEXTN) -> Self {
        Self::ChunkExtn(value)
    }
}

impl From<ChunkSOND> for ChunkOutput {
    fn from(value: ChunkSOND) -> Self {
        Self::ChunkSond(value)
    }
}

impl From<ChunkAGRP> for ChunkOutput {
    fn from(value: ChunkAGRP) -> Self {
        Self::ChunkAgrp(value)
    }
}

impl From<ChunkSPRT> for ChunkOutput {
    fn from(value: ChunkSPRT) -> Self {
        Self::ChunkSprt(value)
    }
}

impl From<ChunkBGND> for ChunkOutput {
    fn from(value: ChunkBGND) -> Self {
        Self::ChunkBgnd(value)
    }
}

impl From<ChunkPATH> for ChunkOutput {
    fn from(value: ChunkPATH) -> Self {
        Self::ChunkPath(value)
    }
}

impl From<ChunkSCPT> for ChunkOutput {
    fn from(value: ChunkSCPT) -> Self {
        Self::ChunkScpt(value)
    }
}

impl From<ChunkGLOB> for ChunkOutput {
    fn from(value: ChunkGLOB) -> Self {
        Self::ChunkGlob(value)
    }
}

impl From<ChunkSHDR> for ChunkOutput {
    fn from(value: ChunkSHDR) -> Self {
        Self::ChunkShdr(value)
    }
}

impl From<ChunkFONT> for ChunkOutput {
    fn from(value: ChunkFONT) -> Self {
        Self::ChunkFont(value)
    }
}

impl From<ChunkTMLN> for ChunkOutput {
    fn from(value: ChunkTMLN) -> Self {
        Self::ChunkTmln(value)
    }
}

impl From<ChunkOBJT> for ChunkOutput {
    fn from(value: ChunkOBJT) -> Self {
        Self::ChunkObjt(value)
    }
}

impl From<ChunkFEDS> for ChunkOutput {
    fn from(value: ChunkFEDS) -> Self {
        Self::ChunkFeds(value)
    }
}

impl From<ChunkACRV> for ChunkOutput {
    fn from(value: ChunkACRV) -> Self {
        Self::ChunkAcrv(value)
    }
}

impl From<ChunkSEQN> for ChunkOutput {
    fn from(value: ChunkSEQN) -> Self {
        Self::ChunkSeqn(value)
    }
}
