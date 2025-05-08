# To-do
- Report that "ReadableConfig" error, which was super unhelpful and very annoying when you don't know why it's bugging.
- Find a better notation for the error-handling in read_config_template() – although the custom error message is desirable, it's very bad
to have to implement it using such a ridiculously long line...
- Refactor so as to allow *either* OAuth2 or password to be filled in, depending on which was entered (with preference for OAuth2).