macro_rules! block_registry {
    ($($name: ident, $id: literal => $body: ident {
        $($fname: ident: $ftyp: ty), *}),*) => {
        0
    };
}

block_registry!(
    SandStone, 24 => SandStone {
        properties: e
    }
);

macro_rules! mac {
    () => {
        0
    };
}
