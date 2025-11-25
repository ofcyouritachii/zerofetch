# Maintainer: ZeroFetch Contributors <zerofetch@example.com>
pkgname=zerofetch
pkgver=0.1.0
pkgrel=1
pkgdesc="A high-performance, modular system information tool for Linux"
arch=('x86_64' 'aarch64')
url="https://github.com/ofcyouritachii/zerofetch"
license=('MIT')
depends=()
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/ofcyouritachii/zerofetch/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
    cd "$srcdir/$pkgname-$pkgver"
    cargo build --release --locked
}

check() {
    cd "$srcdir/$pkgname-$pkgver"
    cargo test --release --locked
}

package() {
    cd "$srcdir/$pkgname-$pkgver"
    
    # Install binary
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
    
    # Install license
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
    
    # Install documentation
    install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
    
    # Install example configs
    install -Dm644 examples/config.jsonc "$pkgdir/usr/share/$pkgname/examples/config.jsonc"
    install -Dm644 examples/config.yaml "$pkgdir/usr/share/$pkgname/examples/config.yaml"
    install -Dm644 examples/config.toml "$pkgdir/usr/share/$pkgname/examples/config.toml"
}
