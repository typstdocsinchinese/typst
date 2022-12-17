use crate::prelude::*;
use crate::text::TextNode;

/// Separate a region into multiple equally sized columns.
///
/// # Parameters
/// - count: usize (positional, required)
///   The number of columns.
/// - body: Content (positional, required)
///   The content that should be layouted into the columns.
///
/// # Tags
/// - layout
#[func]
#[capable(Layout)]
#[derive(Debug, Hash)]
pub struct ColumnsNode {
    /// How many columns there should be.
    pub count: NonZeroUsize,
    /// The child to be layouted into the columns. Most likely, this should be a
    /// flow or stack node.
    pub body: Content,
}

#[node]
impl ColumnsNode {
    /// The size of the gutter space between each column.
    #[property(resolve)]
    pub const GUTTER: Rel<Length> = Ratio::new(0.04).into();

    fn construct(_: &Vm, args: &mut Args) -> SourceResult<Content> {
        Ok(Self {
            count: args.expect("column count")?,
            body: args.expect("body")?,
        }
        .pack())
    }
}

impl Layout for ColumnsNode {
    fn layout(
        &self,
        vt: &mut Vt,
        styles: StyleChain,
        regions: Regions,
    ) -> SourceResult<Fragment> {
        // Separating the infinite space into infinite columns does not make
        // much sense.
        if !regions.first.x.is_finite() {
            return self.body.layout(vt, styles, regions);
        }

        // Determine the width of the gutter and each column.
        let columns = self.count.get();
        let gutter = styles.get(Self::GUTTER).relative_to(regions.base.x);
        let width = (regions.first.x - gutter * (columns - 1) as f64) / columns as f64;

        let backlog: Vec<_> = std::iter::once(&regions.first.y)
            .chain(regions.backlog)
            .flat_map(|&height| std::iter::repeat(height).take(columns))
            .skip(1)
            .collect();

        // Create the pod regions.
        let pod = Regions {
            first: Size::new(width, regions.first.y),
            base: Size::new(width, regions.base.y),
            backlog: &backlog,
            last: regions.last,
            expand: Axes::new(true, regions.expand.y),
        };

        // Layout the children.
        let mut frames = self.body.layout(vt, styles, pod)?.into_iter();
        let mut finished = vec![];

        let dir = styles.get(TextNode::DIR);
        let total_regions = (frames.len() as f32 / columns as f32).ceil() as usize;

        // Stitch together the columns for each region.
        for region in regions.iter().take(total_regions) {
            // The height should be the parent height if we should expand.
            // Otherwise its the maximum column height for the frame. In that
            // case, the frame is first created with zero height and then
            // resized.
            let height = if regions.expand.y { region.y } else { Abs::zero() };
            let mut output = Frame::new(Size::new(regions.first.x, height));
            let mut cursor = Abs::zero();

            for _ in 0..columns {
                let Some(frame) = frames.next() else { break };
                if !regions.expand.y {
                    output.size_mut().y.set_max(frame.height());
                }

                let width = frame.width();
                let x = if dir.is_positive() {
                    cursor
                } else {
                    regions.first.x - cursor - width
                };

                output.push_frame(Point::with_x(x), frame);
                cursor += width + gutter;
            }

            finished.push(output);
        }

        Ok(Fragment::frames(finished))
    }
}

/// A column break.
///
/// # Parameters
/// - weak: bool (named)
///   If true, the column break is skipped if the current column is already
///   empty.
///
/// # Tags
/// - layout
#[func]
#[capable(Behave)]
#[derive(Debug, Hash)]
pub struct ColbreakNode {
    pub weak: bool,
}

#[node]
impl ColbreakNode {
    fn construct(_: &Vm, args: &mut Args) -> SourceResult<Content> {
        let weak = args.named("weak")?.unwrap_or(false);
        Ok(Self { weak }.pack())
    }
}

impl Behave for ColbreakNode {
    fn behaviour(&self) -> Behaviour {
        if self.weak {
            Behaviour::Weak(1)
        } else {
            Behaviour::Destructive
        }
    }
}