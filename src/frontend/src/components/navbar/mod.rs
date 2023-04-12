pub mod dashboard;
pub mod website;

/// contains constant data for menu items in website layout
pub const NAV_ITEMS_WEBSITE: [[&'static str; 2]; 4] = [
    ["Home", "/"],
    ["Blog", "/"],
    ["Performance", "/"],
    ["License", "/"],
];
