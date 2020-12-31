Whatlang
========

Ruby bindings for [Whatlang][], a natural language detection for Rust.

Installation
------------

### Requirements

You need Rust's build environment to install this gem.

For Unix like system, run

    % curl https://sh.rustup.rs -sSf | sh

For Windows, download and run [installer][].

See [Rust official installation page][] for details.

### Gem installation

Add this line to your application's Gemfile:

```ruby
gem 'whatlang'
```

And then execute:

    $ bundle install

Or install it yourself as:

    $ gem install whatlang

Usage
-----

    require "whatlang"
    
    text = "Благодаря Эсперанто вы обрётете друзей по всему миру!"

    info = Whatlang.detect(text)
    info.lang.code     # => "bul"
    info.lang.name     # => "Български"
    info.lang.eng_name # => "Bulgarian"
    info.script        # => "Cyrillic"
    info.confidence    # => 0.2568924409819 (0 to 1 value)
    info.reliable?     # => false
    
    Whatlang.detect_lang(text).code # => "bul"
    Whatlang.detect_script(text)    # => "Cyrillic"
    
    text = "Jen la trinkejo fermitis, ni iras tra mallumo kaj pluvo."
    list = ["eng", "ita"]
    Whatlang.detect(text, whitelist: list).lang.code # => "ita"
    
    text = "Jen la trinkejo fermitis, ni iras tra mallumo kaj pluvo."
    list = ["eng", "ita"]
    Whatlang.detect(text, blacklist: list).lang.code # => "epo"

Development
-----------

After checking out the repo, run `bundle config set --local vendor/bundle && bundle install` to install dependencies. Then, run `bundle exec rake test` to run the tests.

To install this gem onto your local machine, run `bundle exec rake install`. To release a new version, update the version number in `Cargo.toml`, and then run `bundle exec rake release`, which will create a git tag for the version, push git commits and the created tag, and push the `.gem` file to [rubygems.org](https://rubygems.org).

Contributing
------------

Bug reports and pull requests are welcome on GitHub at https://gitlab.com/KitaitiMakoto/whatlang-rb.

License
-------

This RubyGem distributed under the AGPL 3.0 or later. See {file:agpl-3.0.txt} file.

[Whatlang]: https://github.com/greyblake/whatlang-rs
[installer]: https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe
[Rust official installation page]: https://www.rust-lang.org/tools/install
