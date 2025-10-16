use crate::{
    arrays::ArrayBoard,
    bits::{BoardMask, show_mask},
    model::{Direction, Square},
};

#[inline]
pub const fn obstruction_difference(
    neg_ray: BoardMask,
    pos_ray: BoardMask,
    occupied: BoardMask,
) -> BoardMask {
    let neg_hit = neg_ray & occupied;
    let pos_hit = pos_ray & occupied;
    let ms1b = 1u64 << (63 - (neg_hit & occupied | 1).leading_zeros());
    // let ms1b = 1u64 << neg_hit.checked_ilog2().unwrap_or(0);
    let diff = pos_hit ^ pos_hit.wrapping_sub(ms1b);
    return (neg_ray | pos_ray) & diff;
}

pub const fn rank_ray(sq: Square) -> BoardMask {
    sq.file_rank().1.mask()
}

pub const fn file_ray(sq: Square) -> BoardMask {
    sq.file_rank().0.mask()
}

pub const fn diag_ray(sq: Square) -> BoardMask {
    let diag = 0x8040_2010_0804_0201;
    let (f, r) = sq.file_rank();
    let (f, r) = (f.ix(), r.ix());
    let s = r - f;
    if s > 0 {
        diag << (s << 3)
    } else {
        diag >> ((-s) << 3)
    }
}

pub const fn anti_ray(sq: Square) -> BoardMask {
    let anti = 0x0102_0408_1020_4080;
    let (f, r) = sq.file_rank();
    let (f, r) = (f.ix(), 7 - r.ix());
    let s = f - r;
    if s > 0 {
        anti << (s << 3)
    } else {
        anti >> ((-s) << 3)
    }
}

pub const fn split(sq: Square, mask: BoardMask) -> (BoardMask, BoardMask) {
    let (lo, hi) = (!0 >> (63 - sq.ix()) & !sq.bit(), !0 << sq.ix() & !sq.bit());
    (lo & mask, hi & mask)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rays {
    pub rank_rays: (u64, u64),
    pub file_rays: (u64, u64),
    pub diag_rays: (u64, u64),
    pub anti_rays: (u64, u64),
}

impl Rays {
    pub const fn raycasts(sq: Square) -> Self {
        Self {
            rank_rays: split(sq, rank_ray(sq)),
            file_rays: split(sq, file_ray(sq)),
            diag_rays: split(sq, diag_ray(sq)),
            anti_rays: split(sq, anti_ray(sq)),
        }
    }

    #[inline]
    pub fn othrogonal(&self, occupied: BoardMask) -> BoardMask {
        obstruction_difference(self.rank_rays.0, self.rank_rays.1, occupied)
            | obstruction_difference(self.file_rays.0, self.file_rays.1, occupied)
    }

    #[inline]
    pub fn diagonal(&self, occupied: BoardMask) -> BoardMask {
        obstruction_difference(self.diag_rays.0, self.diag_rays.1, occupied)
            | obstruction_difference(self.anti_rays.0, self.anti_rays.1, occupied)
    }

    #[inline]
    pub fn omnidirectional(&self, occupied: BoardMask) -> BoardMask {
        obstruction_difference(self.rank_rays.0, self.rank_rays.1, occupied)
            | obstruction_difference(self.file_rays.0, self.file_rays.1, occupied)
            | obstruction_difference(self.diag_rays.0, self.diag_rays.1, occupied)
            | obstruction_difference(self.anti_rays.0, self.anti_rays.1, occupied)
    }
}

impl ArrayBoard<Rays> {
    pub const fn build() -> Self {
        let mut res = Self::new(Rays {
            rank_rays: (0, 0),
            file_rays: (0, 0),
            diag_rays: (0, 0),
            anti_rays: (0, 0),
        });

        let mut it = Square::a1;

        loop {
            res.set(it, Rays::raycasts(it));

            if let Some(sq) = it.next() {
                it = sq;
            } else {
                break;
            }
        }

        res
    }
}

pub const RAYCASTS: ArrayBoard<Rays> = ArrayBoard::build();
