define(function (require) {
  let Vue = require('vue');

  require('components/patches');
  require('components/graph');
  require('components/menu');
  require('components/info');


//   <b-tabs style="height:100%; flex: 1;" content-class="tabcontent">
//   <b-tab title="Editor" active style="height:100%">
//     <Graph></Graph>
//   </b-tab>
//   <b-tab title="Patches">
//     <Patches></Patches>
//   </b-tab>
// </b-tabs>

  Vue.component('App', {
    template: `
    <div  style="min-height: 100vh">
        <div class="d-flex flex-column" style="min-height:100vh;padding:0px">
          <div>
            <Menu></Menu>
          </div>
          <div class="d-flex flex-column flex-grow-1" style="padding:0px;">
              <Graph></Graph>
          </div>
          <div>
            <Info></Info>
          </div>
        </div>

    </div>`,
    });
});