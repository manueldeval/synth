define(function (require) {
  let helper = require("litegraph/lg_node_helper");

  return function make_knob(type, node,props,commandSender,dirtyFn){

    var knob = function() {
      this.addOutput("","number");
      this.size = [64, 64];
      this.properties = {
          min: -1,
          max: 1,
          color: "#7AF",
          precision: 3,
          ...props
        };
      this.value = -1;
      this.shape = "card";
      this.synth_infos = {
        type: type,
        node_infos: node,
        id: helper.generate_id(type)
      };
      // Put hooks on title and pos
      Object.defineProperty( this, "title", {
        set: function(v) { this._title = v; dirtyFn() },
        get: function() { return this._title; },
        enumerable: true
      });
    }
  
    knob.title = type;
    knob.desc = type;
    knob.size = [60, 80];
  
    knob.prototype.onDrawForeground = function(ctx) {
      if (this.flags.collapsed) {
          return;
      }
  
      if (this.value == -1) {
          this.value =
              (this.properties.value - this.properties.min) /
              (this.properties.max - this.properties.min);
      }
  
      var center_x = this.size[0] * 0.5;
      var center_y = this.size[1] * 0.5;
      var radius = Math.min(this.size[0], this.size[1]) * 0.5 - 5;
      var w = Math.floor(radius * 0.05);
  
      ctx.globalAlpha = 1;
      ctx.save();
      ctx.translate(center_x, center_y);
      ctx.rotate(Math.PI * 0.75);
  
      //bg
      ctx.fillStyle = "rgba(0,0,0,0.5)";
      ctx.beginPath();
      ctx.moveTo(0, 0);
      ctx.arc(0, 0, radius, 0, Math.PI * 1.5);
      ctx.fill();
  
      //value
      ctx.strokeStyle = "black";
      ctx.fillStyle = this.properties.color;
      ctx.lineWidth = 2;
      ctx.beginPath();
      ctx.moveTo(0, 0);
      ctx.arc(
          0,
          0,
          radius - 4,
          0,
          Math.PI * 1.5 * Math.max(0.01, this.value)
      );
      ctx.closePath();
      ctx.fill();
      //ctx.stroke();
      ctx.lineWidth = 1;
      ctx.globalAlpha = 1;
      ctx.restore();
  
      //inner
      ctx.fillStyle = "black";
      ctx.beginPath();
      ctx.arc(center_x, center_y, radius * 0.75, 0, Math.PI * 2, true);
      ctx.fill();
  
      //miniball
      ctx.fillStyle = this.mouseOver ? "white" : this.properties.color;
      ctx.beginPath();
      var angle = this.value * Math.PI * 1.5 + Math.PI * 0.75;
      ctx.arc(
          center_x + Math.cos(angle) * radius * 0.65,
          center_y + Math.sin(angle) * radius * 0.65,
          radius * 0.05,
          0,
          Math.PI * 2,
          true
      );
      ctx.fill();
  
      //text
      ctx.fillStyle = this.mouseOver ? "white" : "#AAA";
      ctx.font = Math.floor(radius * 0.5) + "px Arial";
      ctx.textAlign = "center";
      ctx.fillText(
          this.properties.value.toFixed(this.properties.precision),
          center_x,
          center_y + radius * 0.15
      );
    };
  
    knob.prototype.onExecute = function() {
      this.boxcolor = LiteGraph.colorToString([
          this.value,
          this.value,
          this.value
      ]);
    };
  
    knob.prototype.onMouseDown = function(e) {
      this.center = [this.size[0] * 0.5, this.size[1] * 0.5 + 20];
      this.radius = this.size[0] * 0.5;
      if (
          e.canvasY - this.pos[1] < 20 ||
          LiteGraph.distance(
              [e.canvasX, e.canvasY],
              [this.pos[0] + this.center[0], this.pos[1] + this.center[1]]
          ) > this.radius
      ) {
          return false;
      }
      this.oldmouse = [e.canvasX - this.pos[0], e.canvasY - this.pos[1]];
      this.captureInput(true);
      return true;
    };
  
    knob.prototype.onMouseMove = function(e) {
      if (!this.oldmouse) {
          return;
      }
  
      var m = [e.canvasX - this.pos[0], e.canvasY - this.pos[1]];
  
      var v = this.value;
      v -= (m[1] - this.oldmouse[1]) * 0.01;
      if (v > 1.0) {
          v = 1.0;
      } else if (v < 0.0) {
          v = 0.0;
      }
      this.value = v;
      this.properties.value =
          this.properties.min +
          (this.properties.max - this.properties.min) * this.value;
      helper.sendPropertyConfig.bind(this)("value")
      this.oldmouse = m;
      this.setDirtyCanvas(true);
    };
  
    knob.prototype.onMouseUp = function(e) {
      if (this.oldmouse) {
          this.oldmouse = null;
          this.captureInput(false);
      }
    };
  
    knob.prototype.onPropertyChanged = function(name, value) {
      if (name == "min" || name == "max") {
          this.properties[name] = parseFloat(value);
          return true; //block
      } else {
        var self = this;
        helper.onPropertyChanged.bind(self)(name,value);
      }
    }
    knob.prototype.onExecute = function(){}
    knob.prototype.onAdded = helper.onNodeAdded;
    knob.prototype.onRemoved = helper.onNodeRemoved;
    knob.prototype.onConnectionsChange = helper.onConnectionsChange;
    knob.prototype.sendPropertyConfig = helper.sendPropertyConfig;
    knob.prototype.sendCommand = commandSender;

    return knob;
  }

})