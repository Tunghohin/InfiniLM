#[macro_export]
macro_rules! slice {
    ($blob:expr; $width:expr; [$line:expr]) => {
        $blob[$line as usize * $width as usize..][..$width as usize]
    };
}

mod gather;

use common::utok;
use common_devices::{Operators, SliceOn};
use operators::{
    fuesd_softmax::common_cpu as softmax, mat_mul::common_cpu as mat_mul,
    reform::common_cpu as reform, rms_norm::common_cpu as rms_norm, rope::common_cpu as rope,
    swiglu::common_cpu as swiglu, Operator, QueueOf,
};
use std::ops::{Deref, DerefMut};
use tensor::Tensor;

pub extern crate tensor;

pub use common_devices::{Kernels, KernelsA, KernelsB};
pub use operators::common_cpu::{Handle as Cpu, ThisThread};

pub struct CpuKernels {
    reform: reform::Operator,
    mat_mul: mat_mul::Operator,
    rms_norm: rms_norm::Operator,
    rope: rope::Operator,
    softmax: softmax::Operator,
    swiglu: swiglu::Operator,
}

impl Default for CpuKernels {
    fn default() -> Self {
        Self {
            reform: reform::Operator::new(&Cpu),
            mat_mul: mat_mul::Operator::new(&Cpu),
            rms_norm: rms_norm::Operator::new(&Cpu),
            rope: rope::Operator::new(&Cpu),
            softmax: softmax::Operator::new(&Cpu),
            swiglu: swiglu::Operator::new(&Cpu),
        }
    }
}

impl Kernels<Cpu> for CpuKernels {}

impl Operators for CpuKernels {
    type Handle = Cpu;

    fn reform_op(
        &self,
        _: &QueueOf<Self::Handle>,
    ) -> &impl operators::reform::Reform<Self::Handle> {
        &self.reform
    }
    fn rms_norm_op(
        &self,
        _: &QueueOf<Self::Handle>,
    ) -> &impl operators::rms_norm::RmsNorm<Self::Handle> {
        &self.rms_norm
    }
    fn mat_mul_op(
        &self,
        _: &QueueOf<Self::Handle>,
    ) -> &impl operators::mat_mul::MatMul<Self::Handle> {
        &self.mat_mul
    }
    fn rope_op(&self, _: &QueueOf<Self::Handle>) -> &impl operators::rope::Rope<Self::Handle> {
        &self.rope
    }
    fn softmax_op(
        &self,
        _: &QueueOf<Self::Handle>,
    ) -> &impl operators::fuesd_softmax::FusedSoftmax<Self::Handle> {
        &self.softmax
    }
    fn swiglu_op(
        &self,
        _: &QueueOf<Self::Handle>,
    ) -> &impl operators::swiglu::Swiglu<Self::Handle> {
        &self.swiglu
    }
}

impl KernelsB for CpuKernels {
    type Handle = Cpu;

    fn gather<T, U, I>(
        &self,
        x: &mut Tensor<T>,
        table: &Tensor<U>,
        tokens: I,
        _queue: &QueueOf<Self::Handle>,
    ) where
        T: DerefMut<Target = SliceOn<Self::Handle>>,
        U: Deref<Target = [u8]>,
        I: IntoIterator<Item = utok>,
    {
        gather::gather(x, table, tokens);
    }
}
