
pub struct ADNode {
    n_args: usize,
    m_adjoint: f64,
    p_derivatives: *mut f64,
    p_adj_ptrs: *mut [*mut f64],
}

impl ADNode {
    pub fn new(n_args: usize) -> Self {
        let p_derivatives = std::ptr::null_mut();
        let p_adj_ptrs = Box::into_raw(vec![std::ptr::null_mut(); n_args].into_boxed_slice());
        ADNode {
            n_args,
            m_adjoint: 0.0,
            p_derivatives,
            p_adj_ptrs,
        }
    }

    pub fn adjoint(&self) -> &[*mut f64] {
        unsafe { &*self.p_adj_ptrs }
    }

    pub fn propagate_one(&mut self) {
        if self.n_args == 0 || self.m_adjoint == 0.0 {
            return;
        }
        unsafe {
            for i in 0..self.n_args {
                let v = *self.p_derivatives.add(i) * self.m_adjoint;
                let adj_ptr = *self.p_adj_ptrs.as_mut().unwrap().get_mut(i).unwrap();
                *adj_ptr += v;
            }
        }
    }
}

