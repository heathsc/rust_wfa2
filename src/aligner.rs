use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use super::{
    alignment::Attributes,
    sys::{wavefront_aligner_delete, wavefront_aligner_new, wavefront_aligner_t},
};

pub struct WfaAligner {
    inner: NonNull<wavefront_aligner_t>,
    phantom: PhantomData<wavefront_aligner_t>,
}

impl Drop for WfaAligner {
    fn drop(&mut self) {
        unsafe { wavefront_aligner_delete(self.inner.as_mut()) };
    }
}

impl WfaAligner {
    pub fn new(attributes: &mut Attributes) -> Self {
        let inner = NonNull::new(unsafe { wavefront_aligner_new(attributes) })
            .expect("wf_aligner is NULL");
        Self {
            inner,
            phantom: PhantomData,
        }
    }
}

impl AsRef<wavefront_aligner_t> for WfaAligner {
    fn as_ref(&self) -> &wavefront_aligner_t {
        unsafe { self.inner.as_ref() }
    }
}

impl AsMut<wavefront_aligner_t> for WfaAligner {
    fn as_mut(&mut self) -> &mut wavefront_aligner_t {
        unsafe { self.inner.as_mut() }
    }
}

impl Deref for WfaAligner {
    type Target = wavefront_aligner_t;
    
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl DerefMut for WfaAligner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}