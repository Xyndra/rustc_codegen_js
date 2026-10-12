use dgs::{point, circle, line, polygon, color};

fn main() {
    let mut scene = dgs::Scene::new(dgs::Viewport::new(-5.0, -5.0, 5.0, 5.0, 800.0, 600.0));
    scene = scene.push(point("A", 0.0, 0.0));
    scene = scene.push(circle("A", 2.0));
    scene = scene.push(line("A", "B"));
    println!("{}", scene.to_svg());
}
