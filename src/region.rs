#[derive(Debug, Clone)]
pub struct Region {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl Region {
    pub fn new(w: u32, h: u32) -> Region {
        Region { x: 0, y: 0, w, h }
    }

    pub fn split(self, sub: Region) -> Vec<Region> {
        let w = sub.w;
        let h = sub.h;
        let mut sub_regions = vec![];

        for x in (self.x..self.x + self.w).step_by(sub.w as usize) {
            let mut w = w;
            if x + w > self.x + self.w {
                // Clips the sub-region to within the super-region.
                w = self.x + self.w - x;
            }
            for y in (self.y..self.y + self.h).step_by(sub.h as usize) {
                let mut h = h;
                if y + h > self.y + self.h {
                    h = self.y + self.h - y;
                }
                sub_regions.push(Region { x, y, w, h });
            }
        }

        sub_regions
    }
}
