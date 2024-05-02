use std::{error::Error, fs::File, io::{Write, BufWriter}};

use fad::node::ForwardADNode;
use rand_pcg::Pcg64;
use rand::SeedableRng;
use rand_distr::{Normal, Distribution};

fn force(x: ForwardADNode<f64>, beta: ForwardADNode<f64>) -> ForwardADNode<f64>
{
    -1.0f64 * beta * x
}

fn main() -> Result<(), Box<dyn Error>>
{
    let mut bit_generator = Pcg64::seed_from_u64(0xdeadbeef);
    let distr = Normal::new(0f64, 2f64.sqrt())?;
    let _rval = distr.sample(&mut bit_generator);

    const BETA: f64 = 1.3;
    const N_SAMPLE: usize = 100000;
    const EPS: f64 = 1.0 / 100.0;
    const N_TRAY: u32 = 100;
    const OUTPUT_NAME: &str = "ensemble_ho.npy";

    let mut x0: Vec<f64> = vec![0.0; N_SAMPLE];
    let mut x1: Vec<f64> = vec![0.0; N_SAMPLE];

    let mut x: ForwardADNode<f64> = ForwardADNode{order0: 0f64, order1: 0f64};

    for i in 0..N_SAMPLE
    {
        for _k in 0..N_TRAY
        {
            x = x 
                + EPS * force(x, ForwardADNode{order0: BETA, order1: 1f64}) 
                + EPS.sqrt() * distr.sample(&mut bit_generator);
        }
        x0[i] = x.order0;
        x1[i] = x.order1;
    }
    


    let mut out_file = File::create(OUTPUT_NAME)?;
    let mut out_writer = BufWriter::new(&mut out_file);
    let magic_head = b"\x93NUMPY\x01\x00";
    let descr = b"{'descr': '<f8', 'fortran_order': False, 'shape': (100000, 2), }";

    let header_len = descr.len();
    let total_len = 2 + header_len + magic_head.len() + 1; // +1 for trailing newline
    let mut npad = 0;
    if total_len % 64  != 0
    {
        npad = (total_len / 64 + 1) * 64 - total_len;
    }

    let head_len = ((header_len + npad) as u16).to_le_bytes();

    let pad = b'\x20';
    let mut header: Vec<u8> = magic_head.to_vec();
    header.append(&mut head_len.to_vec());
    header.append(&mut descr.to_vec());
    for _i in 0..npad-1
    {
        header.push(pad);
    }
    header.push(b'\n');

    out_writer.write_all(&header)?;

    
    for (x0i, x1i) in x0.iter().zip(x1)
    {
        out_writer.write_all(&x0i.to_le_bytes())?;
        out_writer.write_all(&x1i.to_le_bytes())?;
    }

    out_writer.flush()?;


    return Ok(());
}
