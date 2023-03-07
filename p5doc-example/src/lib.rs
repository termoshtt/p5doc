#[cfg_attr(doc, p5doc::p5doc)]
/// Some function!
///
/// Before
///
/// <script src="https://cdn.jsdelivr.net/npm/p5@1.6.0/lib/p5.js"></script>
/// <script>
/// function setup() {
///   var canvas = createCanvas(200, 200);
///   canvas.parent("doc-some");
/// }
/// function draw() {
///   background(220);
///   ellipse(50,50,80,80);
/// }
/// </script>
/// <div id="doc-some"></div>
///
/// After
///
pub fn some() {}
