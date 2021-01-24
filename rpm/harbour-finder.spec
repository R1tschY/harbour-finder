Name:       harbour-finder
Summary:    Rust example application for Sailfish OS
Version:    1.0
Release:    1
Group:      Qt/Qt
License:    LICENSE
URL:        http://example.org/
Source0:    %{name}-%{version}.tar.bz2
Requires:   sailfishsilica-qt5 >= 0.10.9
BuildRequires:  pkgconfig(sailfishapp) >= 1.0.2
BuildRequires:  pkgconfig(Qt5Core)
BuildRequires:  pkgconfig(Qt5Qml)
BuildRequires:  pkgconfig(Qt5Quick)
BuildRequires:  desktop-file-utils
BuildRequires:  rust
BuildRequires:  cargo


%description
A example for a Rust application for Sailfish OS.

# - PREP -----------------------------------------------------------------------
%prep
%setup -q -n %{name}-%{version}

# - BUILD ----------------------------------------------------------------------
%build

export RPM_VERSION=%{version}

if [ "$SAILFISH_SDK_FRONTEND" == "qtcreator" ] ; then
    # debug
    export RUSTFLAGS="-Clink-arg=-Wl,-z,relro,-z,now"

    cargo build --target-dir=target --manifest-path %{_sourcedir}/../Cargo.toml
    touch Makefile
else
    # release
    export CARGO_INCREMENTAL=0
    export RUSTFLAGS="-Clink-arg=-Wl,-z,relro,-z,now -Ccodegen-units=1"

    cargo build --release --target-dir=target --locked --manifest-path %{_sourcedir}/../Cargo.toml
fi

# - INSTALL --------------------------------------------------------------------
%install

if [ "$SAILFISH_SDK_FRONTEND" == "qtcreator" ] ; then
    TARGET_BIN_BUILD=target/debug/%{name}
else
    TARGET_BIN_BUILD=target/release/%{name}
fi
SOURCE=%{_sourcedir}/..

rm -rf %{buildroot}
install -d %{buildroot}%{_datadir}/%{name}

install -Dm 755 $TARGET_BIN_BUILD -t %{buildroot}%{_bindir}

install -Dm 644 $SOURCE/res/86x86/%{name}.png -t %{buildroot}%{_datadir}/icons/hicolor/86x86/apps
install -Dm 644 $SOURCE/res/108x108/%{name}.png -t %{buildroot}%{_datadir}/icons/hicolor/108x108/apps
install -Dm 644 $SOURCE/res/128x128/%{name}.png -t %{buildroot}%{_datadir}/icons/hicolor/128x128/apps
install -Dm 644 $SOURCE/res/172x172/%{name}.png -t %{buildroot}%{_datadir}/icons/hicolor/172x172/apps
install -Dm 644 $SOURCE/%{name}.desktop -t %{buildroot}%{_datadir}/applications
cp -r $SOURCE/qml %{buildroot}%{_datadir}/%{name}/qml

desktop-file-install --delete-original       \
  --dir %{buildroot}%{_datadir}/applications             \
   %{buildroot}%{_datadir}/applications/*.desktop

# - FILES ----------------------------------------------------------------------
%files

%defattr(-,root,root,-)
%{_bindir}
%{_datadir}/%{name}/qml
%{_datadir}/applications/%{name}.desktop
%{_datadir}/icons/hicolor/86x86/apps/%{name}.png
%{_datadir}/icons/hicolor/108x108/apps/%{name}.png
%{_datadir}/icons/hicolor/128x128/apps/%{name}.png
%{_datadir}/icons/hicolor/172x172/apps/%{name}.png


