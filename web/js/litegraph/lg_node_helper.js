
define(function (require) {

  let axios = require('axios');

  const MASTER_NODE_TYPE="Master";
  const MASTER_NODE_ID="master";
  const TYPE_TO_VAL = {
    FloatType: { outputType: "FloatVal",  converter: parseFloat},
    IntType: { outputType: "IntVal",      converter: parseInt},
    StringType: { outputType: "StringVal",converter: function(s){return s;}},
    BoolType: { outputType: "BoolVal",    converter: function(s){return s==true}},
  };
  var GLOBAL_COUNTER=0;

  function generate_id(type){
    return (type == MASTER_NODE_TYPE)? MASTER_NODE_ID: type+"#"+(GLOBAL_COUNTER++); 
  }

  function normalize_io_name(name){
    return  name.toLowerCase().replace(/_/g, ' ')
  }

  function sendCommand(command){
    console.log("Sending: ",JSON.stringify(command));
    axios.post('/commands',command)
    .catch(function (error) {
      console.log(error);
    });
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
    node.synth_infos.node_infos.config_spec.forEach(function(p){
      node.sendPropertyConfig(p.key);
    });

  }

  function onPropertyChanged(name,value){
    var conf_spec = this.synth_infos.node_infos.config_spec.find(function(e){
      return e.key == name
    });

    if (conf_spec){
      this.properties[name] = TYPE_TO_VAL[conf_spec.typ].converter(value);
      this.sendPropertyConfig(name);
    }
  }

  function sendPropertyConfig(name){
    var conf_spec = this.synth_infos.node_infos.config_spec.find(function(e){
      return e.key == name
    });
    if (conf_spec){
      val = {};
      val[TYPE_TO_VAL[conf_spec.typ].outputType] = this.properties[name];
      sendCommand({
        ChangeConfig: { 
          id: this.synth_infos.id ,   
          key: name,   
          val: val
        }
      });
    }
  }

  return {
    normalize_io_name,
    generate_id,
    onNodeAdded,
    onNodeRemoved,
    onConnectionsChange,
    onPropertyChanged,
    sendPropertyConfig,
  }

})