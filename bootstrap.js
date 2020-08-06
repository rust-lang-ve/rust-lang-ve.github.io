import '@rust-lang-ve/gh-pages-layout/dist/main.css';

import('./pkg').then((module) => {
  module.run_app();
});
