"use strict";

let rpc = {
  invoke : function(arg) { window.external.invoke(JSON.stringify(arg)); },
  init : function() { rpc.invoke({ cmd : 'init' }); },
  log : function(text_to_log) {
    if (typeof text_to_log !== 'string') {
      text_to_log = JSON.stringify(text_to_log);
    }
    rpc.invoke({cmd: 'log', text: text_to_log});
  },
  render: function(items) {
    // This function is called from the Rust side, we need to update the given elements here ... 
    // return element = picodom.patch(oldNode, (oldNode = UI(items)), element);
  },
  update_process_table_view: function(processes) {
    let process_table_string = '                                            \
        <tr>                                                                \
          <th><p>Name</p></th>                                              \
          <th class="seperator_v movable"></th>                             \
          <th><p>Type</p></th>                                              \
          <th class="seperator_v movable"></th>                             \
          <th><p>Process name</p></th>                                      \
          <th class="seperator_v movable"></th>                             \
          <th><p>Command line</p></th>                                      \
          <th class="seperator_v movable"></th>                             \
          <th class="align_right width_fixed_100 selected_column_blue">     \
            <div>27%</div>                                                  \
            <p>CPU</p>                                                      \
          </th>                                                             \
          <th class="seperator_v"></th>                                     \
          <th class="align_right width_fixed_100">                          \
            <div>27%</div>                                                  \
            <p>Memory</p>                                                   \
          </th>                                                             \
          <th class="seperator_v"></th>                                     \
          <th class="align_right width_fixed_100">                          \
            <div>2%</div>                                                   \
            <p>Disk</p>                                                     \
          </th>                                                             \
          <th class="seperator_v"></th>                                     \
          <th class="align_right width_fixed_100">                          \
            <div>0%</div>                                                   \
            <p>Network</p>                                                  \
          </th>                                                             \
        </tr>                                                               \
    ';
    for (let i = 0; i < processes.length; i++) {
      process_table_string += populate_row_process_table(processes[i]);
    }

    document.getElementById("process_table").innerHTML = process_table_string;
  }
};

function update_process_table() {
    rpc.invoke({ cmd : 'update_process_table' });
}

document.getElementById("end_task").addEventListener("click", update_process_table, false);

setInterval(update_process_table, 1000);

// returns an HTML string for the process table from a process_info struct (see Rust side)
function populate_row_process_table(process_info) {
    let row_html = '<tr>';
    row_html += ('<td class="app_name">' + process_info.name + '</td>');
    row_html += '<td class="seperator_v movable"></td>';
    row_html += ('<td>' + process_info.process_type + '</td>');
    row_html += '<td class="seperator_v movable"></td>';
    row_html += ('<td>' + process_info.process_name + '</td>');
    row_html += '<td class="seperator_v movable"></td>';
    row_html += ('<td>' + process_info.command_line + '</td>');
    row_html += '<td class="seperator_v cpu"></td>';
    row_html += ('<td class="align_right width_fixed_100 light_yellow">' + process_info.cpu_percentage.toFixed(1) + '%' + '</td>');
    row_html += '<td class="seperator_v ram"></td>';
    row_html += ('<td class="align_right width_fixed_100 light_yellow">' + process_info.memory.toFixed(1) + ' MB' + '</td>');
    row_html += '<td class="seperator_v disk"></td>';
    row_html += ('<td class="align_right width_fixed_100 light_yellow">' + Math.round(process_info.disk) + ' MB/s' + '</td>');
    row_html += '<td class="seperator_v network"></td>';
    row_html += ('<td class="align_right width_fixed_100 light_yellow">' + process_info.network.toFixed(1) + ' Mbps' + '</td>');
    row_html += '</tr>';
    return row_html;
}

window.onload = function() { rpc.init(); };