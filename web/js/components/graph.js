define(function (require) {
  let Vue = require('vue');

  let Store = require('store/appstore')
  let registerNodeType = require('litegraph/lg_nodefactory');

  Vue.component('Graph', {
    data:  function () {
      return { 
        nodeTypes: Store.state.nodeTypes
      }
    },
    mounted: function() {
      canvas = new LGraphCanvas(this.$refs.graphCanvas, Store.getGraph(), {autoresize: true});
      canvas.show_info = false;
       

      // Put hooks on pos (to set dirty flag)
      Object.defineProperty( canvas, "node_dragged", {
        set: function(v) { 
          this._node_dragged = v; 
          Store.setDirty(true); 
        },
        get: function() { return this._node_dragged; },
        enumerable: true
      });

      // Overload LGraphCanvas because the heigh is not exacty equals!
      // When the canvas grow, the di grow too :(
      LGraphCanvas.prototype.resize = function(width, height)
      {
        if(!width && !height)
        {
          var parent = this.canvas.parentNode;
          width = parent.offsetWidth;
          height = parent.offsetHeight;          
        }
        if(this.canvas.width > width && Math.abs(this.canvas.height - height) < 10)
          return;
        this.canvas.width = width;
        this.canvas.height = height;
        this.bgcanvas.width = this.canvas.width;
        this.bgcanvas.height = this.canvas.height;
        this.setDirty(true,true);
      }
      canvas.resize();

      canvas.getMenuOptions = function(){ return [ { content:"Audio node", has_submenu: true, callback: LGraphCanvas.onMenuAdd } ] };      
      window.addEventListener("resize", function() { 
        canvas.resize(10,10);
        canvas.resize(); 
      } );
    },
    watch: {
      nodeTypes: function(nodeTypes){
        nodeTypes.forEach(nt => registerNodeType(nt,
          Store.sendCommand.bind(Store),
          () => Store.setDirty(true)
          ));
      }
    },
    template: `
    <div class= "flex-grow-1" style='height:100%;background-color: #8b0000;overflow:hidden '>
      <canvas style="" ref="graphCanvas"></canvas>
    </div>`,  
  });
});