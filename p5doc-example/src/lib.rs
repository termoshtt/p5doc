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
/// ```p5doc:320x280
/// noFill();
/// stroke('gray');
/// strokeWeight(1);
/// beginShape(TRIANGLE_STRIP);
///   vertex(20, 100);
///   vertex(40, 50);
///   vertex(60, 100);
///   vertex(80, 50);
///   vertex(100, 100);
///   vertex(120, 50);
///   vertex(140, 100);
///   vertex(160, 50);
///   vertex(180, 100);
///   vertex(200, 50);
///   vertex(220, 100);
///   vertex(240, 50);
///   vertex(260, 100);
///   vertex(280, 50);
///   vertex(300, 100);
/// endShape();
/// beginShape(TRIANGLE_STRIP);
///   vertex(20, 100);
///   vertex(40, 150);
///   vertex(60, 100);
///   vertex(80, 150);
///   vertex(100, 100);
///   vertex(120, 150);
///   vertex(140, 100);
///   vertex(160, 150);
///   vertex(180, 100);
///   vertex(200, 150);
///   vertex(220, 100);
///   vertex(240, 150);
///   vertex(260, 100);
///   vertex(280, 150);
///   vertex(300, 100);
/// endShape();
/// beginShape(TRIANGLE_STRIP);
///   vertex(20, 200);
///   vertex(40, 150);
///   vertex(60, 200);
///   vertex(80, 150);
///   vertex(100, 200);
///   vertex(120, 150);
///   vertex(140, 200);
///   vertex(160, 150);
///   vertex(180, 200);
///   vertex(200, 150);
///   vertex(220, 200);
///   vertex(240, 150);
///   vertex(260, 200);
///   vertex(280, 150);
///   vertex(300, 200);
/// endShape();
/// beginShape(TRIANGLE_STRIP);
///   vertex(20, 200);
///   vertex(40, 250);
///   vertex(60, 200);
///   vertex(80, 250);
///   vertex(100, 200);
///   vertex(120, 250);
///   vertex(140, 200);
///   vertex(160, 250);
///   vertex(180, 200);
///   vertex(200, 250);
///   vertex(220, 200);
///   vertex(240, 250);
///   vertex(260, 200);
///   vertex(280, 250);
///   vertex(300, 200);
/// endShape();
/// stroke('red');
/// strokeWeight(3);
/// noFill();
/// beginShape();
///   vertex(100, 200);
///   vertex(120, 150);
///   vertex(200, 150);
///   vertex(220, 100);
/// endShape()
/// noStroke();
/// fill('black');
/// circle(100, 200, 10);
/// circle(220, 100, 10);
/// text('start', 100+10, 200+10);
/// text('end', 220+10, 100+10);
/// ```
///
pub fn mesh() {}
