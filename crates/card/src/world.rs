pub struct CardWorldId(u32);

pub struct CardWorld {
    id: CardWorldId,
    name: CardWorldName,
}

pub struct CardWorldName(String);
