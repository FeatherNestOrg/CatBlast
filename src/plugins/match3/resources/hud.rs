use bevy::prelude::*;

#[derive(Resource, Clone, Debug)]
pub struct HudLayout {
    pub window_width: f32,
    pub window_height: f32,

    // 各面板高度/宽度
    pub top_panel_height: f32,
    pub left_panel_width: f32,
    pub right_panel_width: f32,
    pub bottom_panel_height: f32,

    // 中心棋盘可用区域
    pub board_available_width: f32,
    pub board_available_height: f32,
}

impl HudLayout {
    /// 根据分辨率计算响应式布局
    /// 比例：顶部 10%、左右各 25%、底部 15%、中间给棋盘
    pub fn from_resolution(width: f32, height: f32) -> Self {
        let top_panel_height = height * 0.10;
        let left_panel_width = width * 0.25;
        let right_panel_width = width * 0.25;
        let bottom_panel_height = height * 0.15;

        let board_available_width = width - left_panel_width - right_panel_width;
        let board_available_height = height - top_panel_height - bottom_panel_height;

        Self {
            window_width: width,
            window_height: height,
            top_panel_height,
            left_panel_width,
            right_panel_width,
            bottom_panel_height,
            board_available_width,
            board_available_height,
        }
    }

    /// 获取棋盘中心点（世界坐标）
    pub fn get_board_center(&self) -> Vec3 {
        let x = (self.left_panel_width - self.right_panel_width) / 2.0;
        let y = (self.bottom_panel_height - self.top_panel_height) / 2.0;
        Vec3::new(x, y, 0.0)
    }

    /// 棋盘世界坐标边界
    pub fn get_board_bounds(&self) -> (f32, f32, f32, f32) {
        let left = -self.board_available_width / 2.0;
        let right = self.board_available_width / 2.0;
        let bottom = -self.board_available_height / 2.0;
        let top = self.board_available_height / 2.0;
        (left, right, bottom, top)
    }
}

impl Default for HudLayout {
    fn default() -> Self {
        Self::from_resolution(1280.0, 720.0)
    }
}
