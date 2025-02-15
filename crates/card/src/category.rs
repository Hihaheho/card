pub struct CardCategoryId(u32);

pub struct CardCategory {
    id: CardCategoryId,
    name: CardCategoryName,
    description: CardCategoryDescription,
}

pub struct CardCategoryName(String);
pub struct CardCategoryDescription(String);
