# axis

A open source Office-Automation solution.

## Install conan

-   for mac

```bash
$ brew install python cmake
$ pip3 install conan
```

-   for ubuntu

```bash
$ sudo apt-get install python virtualenv build-essential cmake
$ virtualenv python
$ source python/bin/activate
$ pip install conan
$ conan profile new default --detect
$ conan profile update settings.compiler.libcxx=libstdc++11 default
```

## Build

```bash
$ source python/bin/activate
$ mkdir build && cd build
$ conan install .. --build missing
$ cmake ..
$ make -j
```

## Notes

-   Generate a random key

```bash
openssl rand -base64 32
```

## Resources

-   [favicon.ico](http://icoconvert.com/)
-   [smver](http://semver.org/)
-   [keep a changelog](https://keepachangelog.com/en/1.0.0/)
-   [banner.txt](http://patorjk.com/software/taag/)
-   [jwt](https://jwt.io/)
-   [GraphQL](https://graphql.org/learn/)
-   [Alibaba Java Coding Guidelines](https://github.com/alibaba/p3c)
-   [An emoji guide for your commit messages](https://gitmoji.carloscuesta.me/)
-   [Let’s Encrypt](https://letsencrypt.org/)
-   [Certbot](https://certbot.eff.org/)
-   [SSL Server Test](https://www.ssllabs.com/ssltest/index.html)
-   [LINE Developers](https://developers.line.me/en/)
-   [Material Icons](https://material.io/tools/icons/?style=baseline)
-   [Material Design Icons](https://materialdesignicons.com/)
-   [UTF-8 Miscellaneous Symbols](https://www.w3schools.com/charsets/ref_utf_misc_symbols.asp)
-   [UEditor](https://github.com/fex-team/ueditor)
-   [msmtp](https://wiki.archlinux.org/index.php/msmtp)
-   [For gmail smtp](http://stackoverflow.com/questions/20337040/gmail-smtp-debug-error-please-log-in-via-your-web-browser)
