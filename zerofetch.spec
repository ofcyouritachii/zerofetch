Name:           zerofetch
Version:        0.1.0
Release:        1%{?dist}
Summary:        High-performance system information tool for Linux

License:        MIT
URL:            https://github.com/ofcyouritachii/zerofetch
Source0:        %{url}/archive/v%{version}/%{name}-%{version}.tar.gz

BuildRequires:  rust >= 1.70
BuildRequires:  cargo
BuildRequires:  gcc

%description
ZeroFetch is a blazing-fast, modular system information tool written in Rust.
It displays comprehensive system information alongside beautiful ASCII art
logos with extensive customization options.

Features:
* Lightning-fast execution (10-20ms)
* 16 built-in information modules
* Support for all major Linux distributions
* ASCII logos for 15+ distributions
* Multiple output formats (JSON, YAML, TOML)
* Highly customizable via config files

%prep
%autosetup

%build
cargo build --release --locked

%install
install -Dm755 target/release/%{name} %{buildroot}%{_bindir}/%{name}
install -Dm644 LICENSE %{buildroot}%{_datadir}/licenses/%{name}/LICENSE
install -Dm644 README.md %{buildroot}%{_docdir}/%{name}/README.md
install -Dm644 examples/config.jsonc %{buildroot}%{_datadir}/%{name}/examples/config.jsonc
install -Dm644 examples/config.yaml %{buildroot}%{_datadir}/%{name}/examples/config.yaml
install -Dm644 examples/config.toml %{buildroot}%{_datadir}/%{name}/examples/config.toml

%check
cargo test --release --locked

%files
%license LICENSE
%doc README.md
%{_bindir}/%{name}
%{_datadir}/%{name}/

%changelog
* Mon Nov 25 2025 ZeroFetch Contributors <zerofetch@example.com> - 0.1.0-1
- Initial release
- High-performance system information tool for Linux
- 16 built-in information modules
- Support for major Linux distributions
