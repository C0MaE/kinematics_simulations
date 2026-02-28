use plotly::{Plot, Surface};

fn main() {
    let n_phi = 100;
    let n_z = 100;
    let r = 1.0;

    let phi: Vec<f64> = (0..n_phi)
        .map(|i| i as f64 * 2.0 * std::f64::consts::PI / (n_phi as f64 - 1.0))
        .collect();
    let z: Vec<f64> = (0..n_z)
        .map(|i| i as f64 * 5.0 / (n_z as f64 - 1.0))
        .collect();

    // Erzeuge Gitter (phi, z)
    let mut x_data = Vec::new();
    let mut y_data = Vec::new();
    let mut z_data = Vec::new();

    for &phi_i in &phi {
        let mut x_row = Vec::new();
        let mut y_row = Vec::new();
        let mut z_row = Vec::new();

        for &z_i in &z {
            x_row.push(r * phi_i.cos());
            y_row.push(r * phi_i.sin());
            z_row.push(z_i);
        }

        x_data.push(x_row);
        y_data.push(y_row);
        z_data.push(z_row);
    }

    let surface = Surface::new(z_data)
        .x(x_data)
        .y(y_data)
        .name("Zylinder");

    let mut plot = Plot::new();
    plot.add_trace(surface);
    plot.write_html("zylinder.html");
    println!("✅ Zylinder-Plot gespeichert als zylinder.html");
}
