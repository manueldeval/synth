
// Usefull events documentation: https://blokist.gitlab.io/doc/
window.dsp_nodes = {};

const MASTER_NODE_TYPE="Master";
const MASTER_NODE_ID="master";
var GLOBAL_COUNTER=0;

function generate_id(type){
  return (type == MASTER_NODE_TYPE)? MASTER_NODE_ID: type+"#"+(GLOBAL_COUNTER++); 
}

function normalize_io_name(name){
  return  name.toLowerCase().replace(/_/g, ' ')
}

function sendCommand(command){
  console.log("Sending: ",JSON.stringify(command));
}

/*
=================================================
EVENT CALLBACKS
=================================================
*/
function onNodeRemoved(lgraph){
  let node = this;
  if (node.synth_infos.id == MASTER_NODE_ID){
    return;
  }
  sendCommand({"Remove":{"id":node.synth_infos.id}})
}

function onConnectionsChange(io,input_nbr,create,link){
  if (io != LiteGraph.INPUT){
    return;
  }
  var inputNode = this;
  var outputNode = this.lgraph.getNodeById(link.origin_id);
  if (create){
    sendCommand({
      Link: {
        src_node :outputNode.synth_infos.id,  src_port: link.origin_slot,
        dst_node :inputNode.synth_infos.id,  dst_port: input_nbr
      }
    });
  } else {
    sendCommand({
      Unlink: {
        src_node :outputNode.synth_infos.id,  src_port: link.origin_slot,
        dst_node :inputNode.synth_infos.id,  dst_port: input_nbr
      }
    });
  }
}

function onNodeAdded(lgraph){ 
  let node = this;
  node.lgraph = lgraph;
  if (node.synth_infos.id == MASTER_NODE_ID){
    var masterNodes = lgraph.findNodesByType(node.type);
    if (masterNodes.length > 1){
      // There ca be only one master
      lgraph.remove(node);
    } 
    // No need to create server side the master node... it's a default node.
    return;
  }
  sendCommand({"Create":{"id":node.synth_infos.id,"node_type":node.synth_infos.type }});
  // Now send default config
  console.log(node.synth_infos.node_infos.config_spec)
  node.synth_infos.node_infos.config_spec.forEach(function(p){
    node.sendPropertyConfig(p.key);
  });

}

function onPropertyChanged(name,value){
  var conf_spec = this.synth_infos.node_infos.config_spec.find(function(e){
    return e.key == name
  });

  if (conf_spec){
    this.properties[name] = value;
    this.sendPropertyConfig(name);
  }
}

function sendPropertyConfig(name){
  var typeToVal = {
    FloatType: "FloatVal",
    IntType: "IntVal",
    StringType: "StringVal",
    BoolType: "BoolVal"
  };
  var conf_spec = this.synth_infos.node_infos.config_spec.find(function(e){
    return e.key == name
  });
  if (conf_spec){
    val = {};
    val[typeToVal[conf_spec.typ]] = this.properties[name];
    sendCommand({
      ChangeConfig: { 
        id: this.synth_infos.id ,   
        key: name,   
        val: val
      }
    });
  }
}


/*
=================================================
DEFAULT NODE FACTORY
=================================================
*/
function make_default_node(type,node,props){
  var lGraphNodeType = function()
  {
    // Generate i/o for LGraph
    node.io_spec.inputs.forEach(input => {
		  this.addInput(normalize_io_name(input.name),"number");
    });
    node.io_spec.outputs.forEach(input => {
		  this.addOutput(normalize_io_name(input.name),"number");
    });
    this.shape = "card";
    // Save extra informations
    this.synth_infos = {
      type: type,
      node_infos: node,
      id: generate_id(type)
    };
    if (props) {
      this.properties = props
    }
  };
  
  lGraphNodeType.title = type;
  lGraphNodeType.desc = type;
	lGraphNodeType.prototype.onExecute = function(){}
	lGraphNodeType.prototype.onAdded = onNodeAdded;
	lGraphNodeType.prototype.onRemoved = onNodeRemoved;
  lGraphNodeType.prototype.onConnectionsChange = onConnectionsChange;
  lGraphNodeType.prototype.onPropertyChanged = onPropertyChanged;
  lGraphNodeType.prototype.sendPropertyConfig = sendPropertyConfig;
	return lGraphNodeType;

}

function generate_properties(type){
  var props = { 
    Keyboard :  { osc_channel: "/keyboard" },
    Knob:       { osc_channel: "/knob" },
  }
  return props[type] || {}
}

function declareNode(type,node){
  var lGraphNodeType = make_default_node(type, node, generate_properties(type));
  console.log(type);
  LiteGraph.registerNodeType(node.classifier, lGraphNodeType);
}

function fetch_nodes_information(){
  axios.get('/dspnodes').then(function (response) {
    var datas = response.data;
    Object.keys(datas).sort().forEach(function(val){
      declareNode(val,datas[val]);
     });
  })
}
