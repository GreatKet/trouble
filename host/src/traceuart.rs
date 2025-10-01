use embassy_stm32::pac;

macro_rules! writeln {

    ($($arg:tt)*) => {
        #[cfg(feature = "ulog")]
        {
        use core::fmt::Write;
        struct Writer;
        impl Write for Writer {
            fn write_str(&mut self, s: &str) -> core::fmt::Result {
                $crate::traceuart::write(s.as_bytes());
                Ok(())
            }
        }

        let _ = core::write!(Writer, $($arg)*);
        let _ = $crate::traceuart::write(b"\n\r");
        }
        // #[cfg(not(feature = "ulog"))]
        // {let _ = ($( & $arg ),*);}
    };
}

#[cfg(feature = "ulog")]
fn init() {
    pac::RCC
        .d2ccip2r()
        .modify(|m| m.set_usart16910sel(pac::rcc::vals::Usart16910sel::CSI));

    pac::RCC.apb2rstr().modify(|m| m.set_usart1rst(false));
    pac::RCC.apb2enr().modify(|m| m.set_usart1en(true));

    pac::RCC.cr().modify(|m| m.set_csion(true));
    while !pac::RCC.cr().read().csion() {}

    pac::USART1.cr1().write(|w| {
        w.set_ue(false);
    });

    pac::USART1.cr1().write(|w| {
        w.set_uesm(false);
        w.set_re(false);
        w.set_te(true);
        w.set_idleie(false);
        w.set_rxffie(false);
        w.set_tcie(false);
        w.set_txfeie(false);
        w.set_peie(false);
        w.set_pce(false);
        w.set_wake(pac::usart::vals::Wake::IDLE_LINE);
        w.set_m0(pac::usart::vals::M0::BIT8);
        w.set_mme(false);
        w.set_cmie(false);
        w.set_over8(pac::usart::vals::Over8::OVERSAMPLING16);
        w.set_dedt(1);
        w.set_deat(1);
        w.set_rtoie(false);
        w.set_eobie(false);
        w.set_m1(pac::usart::vals::M1::M0);
        w.set_fifoen(false);
        w.set_txfeie(false);
        w.set_rxffie(false);
    });

    pac::USART1.cr2().write(|w| {
        w.set_stop(pac::usart::vals::Stop::STOP1);
        w.set_swap(false);
        w.set_txinv(false);
        w.set_datainv(false);
        w.set_msbfirst(pac::usart::vals::Msbfirst::LSB);
        w.set_abren(false);
    });

    pac::USART1.cr3().write(|w| {
        w.set_eie(false);
        w.set_iren(false);
        w.set_hdsel(false);
        w.set_scen(false);
        w.set_dmar(false);
        w.set_dmat(false);
        w.set_rtse(false);
        w.set_ctse(false);
        w.set_ctsie(false);
        w.set_onebit(false);
        w.set_ovrdis(false);
        w.set_ddre(false);
        w.set_dem(false);
        w.set_txftie(false);
        w.set_rxftie(false);
    });

    pac::USART1.brr().write(|w| w.set_brr(35));

    pac::USART1.cr1().modify(|w| {
        w.set_ue(true);
    });

    pac::GPIOB.afr(0).modify(|m| m.set_afr(6, 7));

    pac::GPIOB
        .moder()
        .modify(|m| m.set_moder(6, pac::gpio::vals::Moder::ALTERNATE));
}

fn write_byte(byte: u8) {
    pac::USART1.icr().write(|w| w.set_tc(true));
    pac::USART1.tdr().write_value(pac::usart::regs::Dr(byte as u32));

    while !pac::USART1.isr().read().tc() {}
}

#[cfg(feature = "ulog")]
pub fn write(data: &[u8]) {
    init();

    for b in data {
        write_byte(*b);
    }
}

#[cfg(not(feature = "ulog"))]
pub fn write(_: &[u8]) {}

pub(crate) use writeln;
