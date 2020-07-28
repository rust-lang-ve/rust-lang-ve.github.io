import './static/tailwind.output.css';

import('./pkg').then((module) => {
  module.run_app();
});
