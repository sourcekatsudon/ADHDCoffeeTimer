#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
#[derive(Debug, Clone, Copy)]
pub struct Target {
    pub rect: Rect,
    pub screen: bool,
}

/// Match adjacent edges and aligned edges only when the other axis overlaps.
pub fn snap(
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    targets: &[Target],
    threshold: i32,
) -> (i32, i32) {
    let (x, y, w, h, t) = (
        x as i64,
        y as i64,
        width as i64,
        height as i64,
        threshold as i64,
    );
    let mut best_x = (t + 1, x);
    let mut best_y = (t + 1, y);
    for target in targets {
        let r = target.rect;
        let (l, top, right, bottom) =
            (r.left as i64, r.top as i64, r.right as i64, r.bottom as i64);
        if y + h >= top - t && y <= bottom + t {
            let candidates = if target.screen {
                vec![l, right - w]
            } else {
                vec![l - w, right, l, right - w]
            };
            for c in candidates {
                let d = (c - x).abs();
                if d <= t && d < best_x.0 {
                    best_x = (d, c);
                }
            }
        }
        if x + w >= l - t && x <= right + t {
            let candidates = if target.screen {
                vec![top, bottom - h]
            } else {
                vec![top - h, bottom, top, bottom - h]
            };
            for c in candidates {
                let d = (c - y).abs();
                if d <= t && d < best_y.0 {
                    best_y = (d, c);
                }
            }
        }
    }
    (
        best_x.1.clamp(i32::MIN as i64, i32::MAX as i64) as i32,
        best_y.1.clamp(i32::MIN as i64, i32::MAX as i64) as i32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn docks_to_work_area_and_negative_monitor() {
        let t = [Target {
            rect: Rect {
                left: -1920,
                top: 0,
                right: 0,
                bottom: 1040,
            },
            screen: true,
        }];
        assert_eq!(snap(-181, 5, 172, 474, &t, 12), (-172, 0));
        assert_eq!(snap(-1915, 560, 172, 474, &t, 12), (-1920, 566));
    }
    #[test]
    fn docks_adjacent_window_edges() {
        let t = [Target {
            rect: Rect {
                left: 500,
                top: 100,
                right: 1000,
                bottom: 800,
            },
            screen: false,
        }];
        assert_eq!(snap(335, 200, 172, 474, &t, 12), (328, 200));
        assert_eq!(snap(1007, 200, 172, 474, &t, 12), (1000, 200));
    }
    #[test]
    fn ignores_distant_windows_and_releases_outside_threshold() {
        let t = [Target {
            rect: Rect {
                left: 500,
                top: 100,
                right: 1000,
                bottom: 800,
            },
            screen: false,
        }];
        assert_eq!(snap(335, 1200, 172, 474, &t, 12), (335, 1200));
        assert_eq!(snap(350, 200, 172, 474, &t, 12), (350, 200));
    }
}
