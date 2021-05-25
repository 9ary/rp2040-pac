impl<U, REG> Reg<U, REG>
where
    Self: Readable + Writable,
    U: Copy + Default + core::ops::Not<Output = U>,
{
    /// Set high every bit in the register that was set in the write proxy. Leave other bits
    /// untouched. The write is done in a single atomic instruction.
    #[inline(always)]
    pub unsafe fn set_bits<F>(&self, f: F)
    where
        F: FnOnce(&mut W<U, Self>) -> &mut W<U, Self>,
    {
        let bits = f(&mut W {
            bits: U::default(),
            _reg: marker::PhantomData,
        })
        .bits;
        let alias = self.register.as_ptr().offset(0x2000 / core::mem::size_of::<U>() as isize)
            as *mut vcell::VolatileCell<U>;
        (*alias).set(bits);
    }

    /// Clear every bit in the register that was cleared in the write proxy. Leave other bits
    /// untouched. The write is done in a single atomic instruction.
    #[inline(always)]
    pub unsafe fn clear_bits<F>(&self, f: F)
    where
        F: FnOnce(&mut W<U, Self>) -> &mut W<U, Self>,
    {
        let bits = f(&mut W {
            bits: !U::default(),
            _reg: marker::PhantomData,
        })
        .bits;
        let alias = self.register.as_ptr().offset(0x3000 / core::mem::size_of::<U>() as isize)
            as *mut vcell::VolatileCell<U>;
        (*alias).set(!bits);
    }

    /// Toggle every bit in the register that was set in the write proxy. Leave other bits
    /// untouched. The write is done in a single atomic instruction.
    #[inline(always)]
    pub unsafe fn toggle_bits<F>(&self, f: F)
    where
        F: FnOnce(&mut W<U, Self>) -> &mut W<U, Self>,
    {
        let bits = f(&mut W {
            bits: U::default(),
            _reg: marker::PhantomData,
        })
        .bits;
        let alias = self.register.as_ptr().offset(0x1000 / core::mem::size_of::<U>() as isize)
            as *mut vcell::VolatileCell<U>;
        (*alias).set(bits);
    }
}
