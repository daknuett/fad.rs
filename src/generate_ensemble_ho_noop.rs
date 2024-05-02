use std::{error::Error, fs::File, io::{Write, BufWriter}};

use fad::node::ForwardADNode;
use rand_pcg::Pcg64;
use rand::SeedableRng;
use rand_distr::{Normal, Distribution};

use clap::Parser;

fn force(x: ForwardADNode<f64>, beta: ForwardADNode<f64>) -> ForwardADNode<f64>
{
    -1.0f64 * beta * x
}

#[derive(Parser)]
struct Cli
{
    beta: f64,
    n_sample: usize,
    eps: f64,
    omega: f64,
    alpha: f64,
    eta_method: String,
    output_name: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn Error>>
{
    let mut bit_generator = Pcg64::seed_from_u64(0xdeadbeef);
    let distr = Normal::new(0f64, 2f64.sqrt())?;
    let _rval = distr.sample(&mut bit_generator);

    let args = Cli::parse();

    let n_tray = (1.0 / args.eps) as u32;
    let eps = args.eps;

    let mut x0: Vec<f64> = vec![0.0; args.n_sample];
    let mut x1: Vec<f64> = vec![0.0; args.n_sample];

    let mut x: ForwardADNode<f64> = ForwardADNode{order0: 0f64, order1: 0f64};

    let eta_method = {
        match args.eta_method.as_str()
        {
            "only0" => 0,
            "independent" => 1,
            "same" => 2,
            _ => 0,
        }
    };


    for i in 0..args.n_sample
    {
        for _k in 0..n_tray
        {
            x = x 
                + eps * force(x, ForwardADNode{order0: args.beta, order1: 1f64}) 
                + {
                    match eta_method
                    {
                        0 => ForwardADNode{order0: eps.sqrt() * distr.sample(&mut bit_generator), order1: 0f64},
                        1 => ForwardADNode{order0: eps.sqrt() * distr.sample(&mut bit_generator), order1: eps.sqrt() * distr.sample(&mut bit_generator)},
                        2 => ForwardADNode{order0: 1f64, order1: 1f64} * eps.sqrt() * distr.sample(&mut bit_generator),
                        _ => ForwardADNode{order0: eps.sqrt() * distr.sample(&mut bit_generator), order1: 0f64},
                    }
                    };
        }
        x0[i] = x.order0;
        x1[i] = x.order1;
    }
    


    let mut out_file = File::create(args.output_name)?;
    let mut out_writer = BufWriter::new(&mut out_file);
    let magic_head = b"\x93NUMPY\x01\x00";
    let mut descr = b"{'descr': '<f8', 'fortran_order': False, 'shape': (".to_vec();
    descr.append(&mut format!("{}", args.n_sample).as_bytes().to_vec());
    descr.append(&mut b", 2), }".to_vec());

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
