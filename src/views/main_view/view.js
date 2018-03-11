let rpc = {
  invoke : function(arg) { window.external.invoke(JSON.stringify(arg)); },
  init : function() { rpc.invoke({ cmd : 'init' }); },
  log : function(text_to_log) {
    if (typeof text_to_log !== 'string') {
      text_to_log = JSON.stringify(text_to_log);
    }
    rpc.invoke({cmd: 'log', text: text_to_log});
  },
  render : function(items) {
    // This function is called from the Rust side, we need to update the given elements here ... 
    // return element = picodom.patch(oldNode, (oldNode = UI(items)), element);
  },

};

document.getElementById("global_menu_file").addEventListener("click", function() {
    rpc.invoke({ cmd : 'update_stuff' });
}, false);

window.onload = function() { rpc.init(); };