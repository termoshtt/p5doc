#[cfg_attr(doc, p5doc::p5doc)]
/// Some function!
///
/// Before
///
/// ```p5doc:200x100
/// background(220);
/// ellipse(50,50,80,80);
/// ```
///
/// After
///
pub fn some() {}

#[cfg_attr(doc, p5doc::p5doc)]
/// Mesh
///
/// ```p5doc:320x320
/// background(220);
/// beginShape(TRIANGLE_STRIP); // outer
///   vertex(100, 140);
///   vertex(120, 50);
///   vertex(180, 120);
///   vertex(240, 60);
///   vertex(210, 140);
///   vertex(260, 130);
///   vertex(220, 180);
///   vertex(230, 240);
///   vertex(150, 230);
///   vertex(130, 250);
///   vertex(120, 180);
///   vertex(80,  200);
///   vertex(100, 140);
///   vertex(60, 100);
///   vertex(120, 50);
/// endShape();
/// beginShape(TRIANGLE_FAN); // inner
///   vertex(160, 160);
///   vertex(100, 140);
///   vertex(180, 120);
///   vertex(210, 140);
///   vertex(220, 180);
///   vertex(150, 230);
///   vertex(120, 180);
///   vertex(100, 140);
/// endShape();
/// ```
///
pub fn mesh() {}
