use crate::{ShellCmdApi, CommonEnv};
use xous_ipc::String;

#[derive(Debug)]
pub struct TrngCmd {
}
impl TrngCmd {
    pub fn new() -> Self {
        TrngCmd {}
    }
}

impl<'a> ShellCmdApi<'a> for TrngCmd {
    cmd_api!(trng);

    fn process(&mut self, args: String::<1024>, env: &mut CommonEnv) -> Result<Option<String::<1024>>, xous::Error> {
        use core::fmt::Write;
        let mut ret = String::<1024>::new();
        let helpstring = "trng [avnist] [ronist] [runs] [excur] [errs] [pump]";

        let mut tokens = args.as_str().unwrap().split(' ');

        if let Some(sub_cmd) = tokens.next() {
            match sub_cmd {
                "avnist" => {
                    let ht = env.trng.get_health_tests().unwrap();
                    write!(ret, "AV NIST stats: {:?}", ht.av_nist).unwrap();
                }
                "ronist" => {
                    let ht = env.trng.get_health_tests().unwrap();
                    write!(ret, "RO NIST stats: {:?}", ht.ro_nist).unwrap();
                }
                "runs" => {
                    let ht = env.trng.get_health_tests().unwrap();
                    for core in 0..4 {
                        write!(ret, "RO {}: ", core).unwrap();
                        for bin in 0..4 {
                            write!(ret, "{} ", ht.ro_miniruns[core].run_count[bin]).unwrap();
                        }
                        write!(ret, "\n").unwrap();
                    }
                }
                "excur" => {
                    let ht = env.trng.get_health_tests().unwrap();
                    write!(ret, "AV0: {:.0}/{:.0} mV\n"
                        ,(ht.av_excursion[0].min as f64 / 4096.0) * 1000.0
                        ,(ht.av_excursion[0].max as f64 / 4096.0) * 1000.0
                    ).unwrap();
                    write!(ret, "AV0 delta: {:.0} mV\n"
                        ,((ht.av_excursion[0].max as f64 - ht.av_excursion[0].min as f64) / 4096.0) * 1000.0
                    ).unwrap();
                    write!(ret, "AV1: {:.0}/{:.0} mV\n"
                        ,(ht.av_excursion[1].min as f64 / 4096.0) * 1000.0
                        ,(ht.av_excursion[1].max as f64 / 4096.0) * 1000.0
                    ).unwrap();
                    write!(ret, "AV1 delta: {:.0} mV\n"
                        ,((ht.av_excursion[1].max as f64 - ht.av_excursion[1].min as f64) / 4096.0) * 1000.0
                    ).unwrap();
                }
                "pump" => {
                    const ROUNDS: usize = 16;
                    for i in 0..ROUNDS {
                        log::debug!("pump round {}", i);
                        let mut buf: [u32; 1024] = [0; 1024];
                        env.trng.fill_buf(&mut buf).unwrap();
                        log::debug!("pump samples: {:x}, {:x}, {:x}", buf[0], buf[512], buf[1023]);
                    }
                    write!(ret, "Pumped {}x1k values out of the engine", ROUNDS).unwrap();
                }
                "errs" => {
                    write!(ret, "TRNG error stats: {:?}", env.trng.get_error_stats().unwrap()).unwrap();
                }
                _ => {
                    write!(ret, "{}", helpstring).unwrap();
                }
            }

        } else {
            write!(ret, "{}", helpstring).unwrap();
        }
        Ok(Some(ret))
    }
}
