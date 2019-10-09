define(function (require) {
  let Vue = require('vue');

  require('components/greeting');
  require('components/graph');
  require('components/menu');

  Vue.component('App', {
    template: `<div>
      <Menu></Menu>
      <Greeting></Greeting>
      <Graph></Graph>
    </div>`,
    });
});