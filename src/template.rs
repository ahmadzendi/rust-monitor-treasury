pub const HTML_TEMPLATE: &str = r##"<!DOCTYPE html>
<html lang="id">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width,initial-scale=1,maximum-scale=5">
<title>Harga Emas Treasury</title>
<link rel="stylesheet" href="https://cdn.datatables.net/1.13.6/css/jquery.dataTables.min.css"/>
<style>
*{box-sizing:border-box}
body{font-family:Arial,sans-serif;margin:0;padding:5px 20px 0 20px;background:#fff;color:#222;transition:background .3s,color .3s}
h2{margin:0 0 2px}
h3{margin:20px 0 10px}
.header{display:flex;align-items:center;justify-content:space-between;gap:10px;margin-bottom:2px}
.title-wrap{display:flex;align-items:center;gap:10px}
.tele-link{display:inline-flex;align-items:center;gap:6px;text-decoration:none;transition:transform .2s}
.tele-link:hover{transform:scale(1.05)}
.tele-icon{display:inline-flex;align-items:center;justify-content:center;width:32px;height:32px;background:#0088cc;color:#fff;border-radius:50%;transition:background .3s}
.tele-link:hover .tele-icon{background:#006699}
.tele-text{font-size:0.95em;font-weight:bold;color:#ff1744}
.dark-mode .tele-icon{background:#29b6f6}
.dark-mode .tele-link:hover .tele-icon{background:#0288d1}
.dark-mode .tele-text{color:#00E124}
#jam{font-size:2em;color:#ff1744;font-weight:bold;margin-bottom:8px}
table.dataTable{width:100%!important;border-collapse:collapse}
table.dataTable thead th{font-weight:bold;white-space:nowrap;padding:10px 8px;font-size:1em;border-bottom:2px solid #ddd}
table.dataTable tbody td{padding:8px 6px;white-space:nowrap;border-bottom:1px solid #eee;font-size:1em}
th.waktu,td.waktu{width:78px;min-width:72px;max-width:82px;text-align:center;padding-left:2px!important;padding-right:2px!important}
th.transaksi,td.transaksi{text-align:left;min-width:220px}
th.profit,td.profit{width:155px;min-width:145px;max-width:165px;text-align:left;padding-left:8px!important;padding-right:8px!important}
.theme-toggle-btn{padding:0;border:none;border-radius:50%;background:#222;color:#fff;cursor:pointer;font-size:1.5em;width:44px;height:44px;display:flex;align-items:center;justify-content:center;transition:background .3s}
.theme-toggle-btn:hover{background:#444}
.dark-mode{background:#181a1b!important;color:#e0e0e0!important}
.dark-mode #jam{color:#ffb300!important}
.dark-mode table.dataTable,.dark-mode table.dataTable thead th{background:#23272b!important;color:#e0e0e0!important}
.dark-mode table.dataTable tbody td{background:#23272b;color:#e0e0e0!important;border-bottom:1px solid #333}
.dark-mode table.dataTable thead th{color:#ffb300!important;border-bottom:2px solid #444}
.dark-mode .theme-toggle-btn{background:#ffb300;color:#222}
.dark-mode .theme-toggle-btn:hover{background:#ffd54f}
.container-flex{display:flex;gap:15px;flex-wrap:wrap;margin-top:10px}
.card{border:1px solid #ccc;border-radius:6px;padding:10px}
.card-usd{width:248px;height:370px;overflow-y:auto}
.card-chart{flex:1;min-width:400px;height:370px;overflow:hidden}
#priceList{list-style:none;padding:0;margin:0;max-height:275px;overflow-y:auto}
#priceList li{margin-bottom:1px}
.time{color:gray;font-size:.9em;margin-left:10px}
#currentPrice{color:red;font-weight:bold}
.dark-mode #currentPrice{color:#00E124;text-shadow:1px 1px #00B31C}
#tabel tbody tr:first-child td{color:red!important;font-weight:bold}
.dark-mode #tabel tbody tr:first-child td{color:#00E124!important}
#footerApp{width:100%;position:fixed;bottom:0;left:0;background:transparent;text-align:center;z-index:100;padding:8px 0}
.marquee-text{display:inline-block;color:#F5274D;animation:marquee 70s linear infinite;font-weight:bold}
.dark-mode .marquee-text{color:#B232B2}
@keyframes marquee{0%{transform:translateX(100vw)}100%{transform:translateX(-100%)}}
.loading-text{color:#999;font-style:italic}
.tbl-wrap{width:100%;overflow-x:auto;-webkit-overflow-scrolling:touch}
.dataTables_wrapper{position:relative}
.dt-top-controls{display:flex;justify-content:space-between;align-items:center;flex-wrap:wrap;gap:8px;margin-bottom:0!important;padding:8px 0;padding-bottom:0!important}
.dataTables_wrapper .dataTables_length{margin:0!important;float:none!important;margin-bottom:0!important;padding-bottom:0!important}
.dataTables_wrapper .dataTables_filter{margin:0!important;float:none!important}
.dataTables_wrapper .dataTables_info{display:none!important}
.dataTables_wrapper .dataTables_paginate{margin-top:10px!important;text-align:center!important}
.tbl-wrap{margin-top:0!important;padding-top:0!important}
#tabel.dataTable{margin-top:0!important}
#tabel tbody td.transaksi{padding:6px 8px;white-space:nowrap}
.profit-order-btns{display:none;gap:3px;align-items:center;margin-right:6px}
.profit-btn{padding:5px 10px;border:1px solid #aaa;background:#f0f0f0;border-radius:4px;font-size:12px;cursor:pointer;font-weight:bold;transition:all .2s}
.profit-btn:hover{background:#ddd}
.profit-btn.active{background:#007bff;color:#fff;border-color:#007bff}
.dark-mode .profit-btn{background:#333;border-color:#555;color:#ccc}
.dark-mode .profit-btn:hover{background:#444}
.dark-mode .profit-btn.active{background:#ffb300;color:#222;border-color:#ffb300}
.filter-wrap{display:flex;align-items:center}
.tradingview-wrapper{height:100%;width:100%;overflow:hidden}
.chart-header{display:flex;justify-content:space-between;align-items:center;margin-top:0;margin-bottom:10px}
.chart-header h3{margin:0}
.limit-label{font-size:0.95em;font-weight:bold;color:#ff1744}
.limit-label .limit-num{font-size:1.1em;padding:2px 8px;background:#ff1744;color:#fff;border-radius:4px;margin-left:4px}
.dark-mode .limit-label{color:#00E124}
.dark-mode .limit-label .limit-num{background:#00E124;color:#181a1b}
.dark-mode .card{border-color:#444}
#tabel thead th.waktu,#tabel tbody td.waktu{position:sticky;left:0;z-index:2;background:#fff}
#tabel thead th.waktu{z-index:3}
.dark-mode #tabel thead th.waktu{background:#23272b}
.dark-mode #tabel tbody td.waktu{background:#23272b}
@keyframes blink-yellow{0%,100%{background-color:#fff}50%{background-color:#ffeb3b}}
@keyframes blink-yellow-dark{0%,100%{background-color:#23272b}50%{background-color:#ffd600}}
#tabel tbody tr.blink-row td.waktu{animation:blink-yellow 0.4s ease-in-out 5}
.dark-mode #tabel tbody tr.blink-row td.waktu{animation:blink-yellow-dark 0.4s ease-in-out 5}
.bottom-section{display:flex;flex-direction:row;gap:20px;margin-top:20px;margin-bottom:60px;align-items:flex-start}
.calendar-box{flex:1;min-width:0}
.calendar-box h3{margin:0 0 10px;font-size:1.1em;display:flex;align-items:center;gap:8px}
.card-calendar{border:1px solid #ccc;border-radius:6px;padding:10px;height:520px;overflow:hidden}
.dark-mode .card-calendar{border-color:#444;background:#23272b}
.calendar-scroll-wrapper{width:100%;height:calc(100% - 30px);overflow-x:auto;overflow-y:hidden;-webkit-overflow-scrolling:touch}
.calendar-scroll-wrapper iframe{min-width:650px;width:100%;height:100%;border:none}
.calendar-credit{font-family:Arial,Helvetica,sans-serif;font-size:11px;color:#333;margin-top:8px;text-align:center}
.calendar-credit a{color:#06529D;font-weight:bold;text-decoration:none}
.calendar-credit a:hover{text-decoration:underline}
.dark-mode .calendar-credit{color:#aaa}
.dark-mode .calendar-credit a{color:#ffb300}
.calc-box{width:300px;flex-shrink:0}
.calc-box h3{margin:0 0 10px;font-size:1.1em}
.calc-cards{display:flex;flex-direction:column;gap:12px}
.card-calc{border:1px solid #ccc;border-radius:6px;padding:14px;background:#fff}
.dark-mode .card-calc{border-color:#444;background:#23272b}
.card-calc h4{margin:0 0 10px;font-size:1em;display:flex;align-items:center;gap:6px;color:#333}
.dark-mode .card-calc h4{color:#e0e0e0}
.calc-rate{font-size:0.9em;color:#666;margin:0 0 12px;padding:8px 10px;background:#f5f5f5;border-radius:4px;font-weight:500}
.calc-rate span{font-weight:bold;color:#ff1744}
.dark-mode .calc-rate{background:#2a2e32;color:#aaa}
.dark-mode .calc-rate span{color:#00E124}
.calc-input-group{margin-bottom:12px}
.calc-input-group:last-child{margin-bottom:0}
.calc-input-group label{display:block;font-size:0.85em;margin-bottom:5px;color:#555;font-weight:500}
.dark-mode .calc-input-group label{color:#aaa}
.calc-input-group input{width:100%;padding:12px;border:2px solid #ddd;border-radius:6px;font-size:1em;transition:border-color .2s,box-shadow .2s}
.calc-input-group input:focus{outline:none;border-color:#007bff;box-shadow:0 0 0 3px rgba(0,123,255,0.15)}
.dark-mode .calc-input-group input{background:#1e2124;border-color:#444;color:#e0e0e0}
.dark-mode .calc-input-group input:focus{border-color:#ffb300;box-shadow:0 0 0 3px rgba(255,179,0,0.15)}
.calc-input-group input::placeholder{color:#999}
.dark-mode .calc-input-group input::placeholder{color:#666}
@media(min-width:768px) and (max-width:1024px){
body{padding:15px;padding-bottom:50px}
h2{font-size:1.15em}
h3{font-size:1.05em;margin:15px 0 8px}
.header{margin-bottom:4px}
.tele-icon{width:30px;height:30px}
.tele-icon svg{width:16px;height:16px}
.tele-text{font-size:0.9em}
#jam{font-size:2em;margin-bottom:8px}
.theme-toggle-btn{width:42px;height:42px;font-size:1.4em}
.container-flex{flex-direction:row;gap:15px}
.card-usd{width:220px;height:350px}
.card-chart{flex:1;min-width:350px;height:350px}
.dt-top-controls{flex-direction:row;justify-content:space-between;gap:8px;margin-bottom:8px;padding:6px 0}
.dataTables_wrapper .dataTables_length{font-size:14px!important}
.dataTables_wrapper .dataTables_filter{font-size:14px!important}
.dataTables_wrapper .dataTables_filter input{width:100px!important;font-size:14px!important;padding:5px 8px!important}
.dataTables_wrapper .dataTables_length select{font-size:14px!important;padding:4px!important}
.dataTables_wrapper .dataTables_paginate .paginate_button{padding:6px 14px!important;font-size:14px!important}
#tabel{min-width:1000px!important;table-layout:fixed!important}
#tabel thead th{font-size:15px!important;padding:10px 6px!important;font-weight:bold!important}
#tabel tbody td{font-size:14px!important;padding:9px 5px!important}
#tabel thead th.waktu,#tabel tbody td.waktu{width:80px!important;min-width:75px!important;max-width:85px!important;padding-left:3px!important;padding-right:3px!important}
#tabel thead th.transaksi,#tabel tbody td.transaksi{width:250px!important;min-width:245px!important;max-width:255px!important;padding:8px 10px!important}
#tabel thead th.profit,#tabel tbody td.profit{width:130px!important;min-width:125px!important;max-width:135px!important;padding-left:6px!important;padding-right:6px!important}
.profit-order-btns{display:flex}
.profit-btn{padding:6px 12px;font-size:13px}
.chart-header{flex-direction:row;gap:10px}
.chart-header h3{font-size:1em}
.limit-label{font-size:0.9em}
.limit-label .limit-num{font-size:1.05em;padding:2px 7px}
.bottom-section{flex-direction:row;gap:15px}
.calendar-box{flex:1;min-width:0}
.calc-box{width:280px}
.card-calendar{height:500px}
.calc-cards{flex-direction:column;gap:10px}
.calc-cards .card-calc{padding:12px}
.card-calc h4{font-size:0.95em}
.calc-rate{font-size:0.85em;padding:6px 8px}
.calc-input-group input{padding:10px;font-size:0.95em}
}
@media(min-width:576px) and (max-width:767px){
body{padding:12px;padding-bottom:50px}
h2{font-size:1.05em}
h3{font-size:0.95em;margin:12px 0 8px}
.header{margin-bottom:2px}
.tele-icon{width:28px;height:28px}
.tele-icon svg{width:15px;height:15px}
.tele-text{font-size:0.85em}
#jam{font-size:1.8em;margin-bottom:6px}
.theme-toggle-btn{width:38px;height:38px;font-size:1.3em}
.container-flex{flex-direction:column;gap:15px}
.card-usd,.card-chart{width:100%!important;max-width:100%!important;min-width:0!important}
.card-usd{height:auto;min-height:300px}
.card-chart{height:360px}
.dt-top-controls{flex-direction:row;justify-content:space-between;gap:5px;margin-bottom:8px;padding:5px 0}
.dataTables_wrapper .dataTables_length{font-size:13px!important}
.dataTables_wrapper .dataTables_filter{font-size:13px!important}
.dataTables_wrapper .dataTables_filter input{width:85px!important;font-size:13px!important;padding:4px 6px!important}
.dataTables_wrapper .dataTables_length select{font-size:13px!important;padding:3px!important}
.dataTables_wrapper .dataTables_paginate .paginate_button{padding:5px 12px!important;font-size:13px!important}
#tabel{min-width:950px!important;table-layout:fixed!important}
#tabel thead th{font-size:14px!important;padding:9px 5px!important;font-weight:bold!important}
#tabel tbody td{font-size:13px!important;padding:8px 4px!important}
#tabel thead th.waktu,#tabel tbody td.waktu{width:75px!important;min-width:70px!important;max-width:80px!important}
#tabel thead th.transaksi,#tabel tbody td.transaksi{width:235px!important;min-width:230px!important;max-width:240px!important;padding:7px 8px!important}
#tabel thead th.profit,#tabel tbody td.profit{width:125px!important;min-width:120px!important;max-width:130px!important;padding-left:5px!important;padding-right:5px!important}
.profit-order-btns{display:flex}
.profit-btn{padding:5px 10px;font-size:12px}
.chart-header{flex-direction:row;gap:8px}
.chart-header h3{font-size:0.95em}
.limit-label{font-size:0.85em}
.bottom-section{flex-direction:column;gap:18px;margin-bottom:55px}
.calendar-box{width:100%}
.calc-box{width:100%}
.card-calendar{height:480px}
.calendar-box h3{font-size:1em}
.calc-cards{flex-direction:row;gap:12px}
.calc-cards .card-calc{flex:1;padding:12px}
.calc-box h3{font-size:1em}
}
@media(min-width:480px) and (max-width:575px){
body{padding:10px;padding-bottom:48px}
h2{font-size:1em}
h3{font-size:0.92em;margin:12px 0 6px}
.header{margin-bottom:2px}
.title-wrap{gap:6px}
.tele-icon{width:26px;height:26px}
.tele-icon svg{width:14px;height:14px}
.tele-text{font-size:0.8em}
#jam{font-size:1.5em;margin-bottom:5px}
.theme-toggle-btn{width:36px;height:36px;font-size:1.2em}
.container-flex{flex-direction:column;gap:12px}
.card-usd,.card-chart{width:100%!important;max-width:100%!important;min-width:0!important}
.card-usd{height:auto;min-height:280px}
.card-chart{height:340px}
.card{padding:8px}
#footerApp{padding:6px 0}
.marquee-text{font-size:12px}
.dt-top-controls{gap:4px;margin-bottom:6px}
.dataTables_wrapper .dataTables_length,.dataTables_wrapper .dataTables_filter{font-size:12px!important}
.dataTables_wrapper .dataTables_filter input{width:75px!important;font-size:12px!important}
.dataTables_wrapper .dataTables_length select{font-size:12px!important}
.dataTables_wrapper .dataTables_paginate .paginate_button{padding:5px 10px!important;font-size:12px!important}
#priceList{max-height:220px}
#tabel{min-width:900px!important;table-layout:fixed!important}
#tabel thead th{font-size:13px!important;padding:8px 4px!important;font-weight:bold!important}
#tabel tbody td{font-size:12px!important;padding:7px 3px!important}
#tabel thead th.waktu,#tabel tbody td.waktu{width:72px!important;min-width:68px!important;max-width:76px!important}
#tabel thead th.transaksi,#tabel tbody td.transaksi{width:220px!important;min-width:215px!important;max-width:225px!important;padding:6px 6px!important}
#tabel thead th.profit,#tabel tbody td.profit{width:118px!important;min-width:113px!important;max-width:123px!important;padding-left:4px!important;padding-right:4px!important}
.profit-order-btns{display:flex}
.profit-btn{padding:5px 9px;font-size:11px}
.chart-header h3{font-size:0.9em}
.limit-label{font-size:0.82em}
.limit-label .limit-num{font-size:1em;padding:1px 6px}
.bottom-section{flex-direction:column;gap:15px;margin-bottom:50px}
.calendar-box{width:100%}
.calc-box{width:100%}
.card-calendar{height:450px}
.calendar-box h3{font-size:0.95em}
.calc-cards{flex-direction:row;gap:10px}
.calc-cards .card-calc{flex:1;padding:10px}
.card-calc h4{font-size:0.9em;margin-bottom:8px}
.calc-rate{font-size:0.8em;padding:6px 8px;margin-bottom:10px}
.calc-input-group{margin-bottom:10px}
.calc-input-group label{font-size:0.8em}
.calc-input-group input{padding:10px;font-size:0.95em}
.calc-box h3{font-size:1em}
}
@media(max-width:479px){
body{padding:8px;padding-bottom:45px}
h2{font-size:0.9em}
h3{font-size:0.88em;margin:10px 0 6px}
.header{margin-bottom:1px}
.title-wrap{gap:5px}
.tele-icon{width:24px;height:24px}
.tele-icon svg{width:13px;height:13px}
.tele-text{font-size:0.75em}
#jam{font-size:1.3em;margin-bottom:4px}
.theme-toggle-btn{width:34px;height:34px;font-size:1.1em}
.container-flex{flex-direction:column;gap:10px}
.card-usd,.card-chart{width:100%!important;max-width:100%!important;min-width:0!important}
.card-usd{height:auto;min-height:260px}
.card-chart{height:300px}
.card{padding:6px}
#footerApp{padding:5px 0}
.marquee-text{font-size:11px}
.dt-top-controls{gap:3px;margin-bottom:5px}
.dataTables_wrapper .dataTables_length,.dataTables_wrapper .dataTables_filter{font-size:11px!important}
.dataTables_wrapper .dataTables_filter input{width:60px!important;font-size:11px!important}
.dataTables_wrapper .dataTables_length select{font-size:11px!important}
.dataTables_wrapper .dataTables_paginate .paginate_button{padding:4px 8px!important;font-size:11px!important}
#priceList{max-height:180px}
#tabel{min-width:850px!important;table-layout:fixed!important}
#tabel thead th{font-size:12px!important;padding:7px 3px!important;font-weight:bold!important}
#tabel tbody td{font-size:11px!important;padding:6px 3px!important}
#tabel thead th.waktu,#tabel tbody td.waktu{width:68px!important;min-width:64px!important;max-width:72px!important;padding-left:2px!important;padding-right:2px!important}
#tabel thead th.transaksi,#tabel tbody td.transaksi{width:210px!important;min-width:205px!important;max-width:215px!important;padding:5px 5px!important}
#tabel thead th.profit,#tabel tbody td.profit{width:110px!important;min-width:105px!important;max-width:115px!important;padding-left:3px!important;padding-right:3px!important}
.profit-order-btns{display:flex}
.profit-btn{padding:4px 7px;font-size:10px}
.chart-header h3{font-size:0.85em}
.limit-label{font-size:0.78em}
.limit-label .limit-num{font-size:0.95em;padding:1px 5px}
.bottom-section{flex-direction:column;gap:15px;margin-bottom:48px}
.calendar-box{width:100%}
.calc-box{width:100%}
.card-calendar{height:420px}
.calendar-box h3{font-size:0.9em}
.calc-cards{flex-direction:column;gap:10px}
.calc-cards .card-calc{width:100%;padding:10px}
.card-calc h4{font-size:0.88em;gap:4px;margin-bottom:8px}
.calc-rate{font-size:0.78em;padding:6px 8px;margin-bottom:8px}
.calc-input-group{margin-bottom:8px}
.calc-input-group label{font-size:0.78em;margin-bottom:4px}
.calc-input-group input{padding:10px;font-size:0.9em;border-radius:5px}
.calc-box h3{font-size:0.95em}
}
</style>
</head>
<body>
<div class="header">
<div class="title-wrap">
<h2>Harga Emas Treasury  ➺ </h2>
<a href="https://t.me/+FLtJjyjVV8xlM2E1" target="_blank" class="tele-link" title="Join Telegram"><span class="tele-icon"><svg viewBox="0 0 24 24" width="18" height="18" fill="currentColor"><path d="M11.944 0A12 12 0 0 0 0 12a12 12 0 0 0 12 12 12 12 0 0 0 12-12A12 12 0 0 0 12 0a12 12 0 0 0-.056 0zm4.962 7.224c.1-.002.321.023.465.14a.506.506 0 0 1 .171.325c.016.093.036.306.02.472-.18 1.898-.962 6.502-1.36 8.627-.168.9-.499 1.201-.82 1.23-.696.065-1.225-.46-1.9-.902-1.056-.693-1.653-1.124-2.678-1.8-1.185-.78-.417-1.21.258-1.91.177-.184 3.247-2.977 3.307-3.23.007-.032.014-.15-.056-.212s-.174-.041-.249-.024c-.106.024-1.793 1.14-5.061 3.345-.48.33-.913.49-1.302.48-.428-.008-1.252-.241-1.865-.44-.752-.245-1.349-.374-1.297-.789.027-.216.325-.437.893-.663 3.498-1.524 5.83-2.529 6.998-3.014 3.332-1.386 4.025-1.627 4.476-1.635z"/></svg></span><span class="tele-text">Telegram</span></a>
</div>
<button class="theme-toggle-btn" id="themeBtn" onclick="toggleTheme()" title="Ganti Tema">🌙</button>
</div>
<div id="jam"></div>
<div class="tbl-wrap">
<table id="tabel" class="display">
<thead>
<tr>
<th class="waktu">Waktu</th>
<th class="transaksi">Data Transaksi</th>
<th class="profit" id="thP1">Est.cuan 10JT ➺ gr</th>
<th class="profit" id="thP2">Est.cuan 20JT ➺ gr</th>
<th class="profit" id="thP3">Est.cuan 30JT ➺ gr</th>
<th class="profit" id="thP4">Est.cuan 40JT ➺ gr</th>
<th class="profit" id="thP5">Est.cuan 50JT ➺ gr</th>
</tr>
</thead>
<tbody></tbody>
</table>
</div>
<div class="container-flex">
<div style="flex:1;min-width:400px">
<div class="chart-header">
<h3>Chart Harga Emas (XAU/USD)</h3>
<span class="limit-label">Limit Bulan ini:<span class="limit-num" id="limitBulan">88888</span></span>
</div>
<div class="card card-chart">
<div class="tradingview-wrapper" id="tradingview_chart"></div>
</div>
</div>
<div>
<h3 style="margin-top:0">Harga USD/IDR Google Finance</h3>
<div class="card card-usd">
<p>Harga saat ini: <span id="currentPrice" class="loading-text">Memuat data...</span></p>
<h4>Harga Terakhir:</h4>
<ul id="priceList"><li class="loading-text">Menunggu data...</li></ul>
</div>
</div>
</div>
<div class="bottom-section">
<div class="calendar-box">
<h3>📅 Kalender Ekonomi</h3>
<div class="card card-calendar">
<div class="calendar-scroll-wrapper">
<iframe src="https://sslecal2.investing.com?columns=exc_flags,exc_currency,exc_importance,exc_actual,exc_forecast,exc_previous&category=_employment,_economicActivity,_inflation,_credit,_centralBanks,_confidenceIndex,_balance,_Bonds&importance=3&features=datepicker,timezone,timeselector,filters&countries=5,25,37,48,4,35,17,22,12,72&calType=week&timeZone=27&lang=54" allowtransparency="true" marginwidth="0" marginheight="0"></iframe>
</div>
<div class="calendar-credit">
<span>Kalender Ekonomi Real Time dipersembahkan oleh <a href="https://id.investing.com" rel="nofollow" target="_blank">Investing.com Indonesia</a>.</span>
</div>
</div>
</div>
<div class="calc-box">
<h3>🧮 Kalkulator Emas</h3>
<div class="calc-cards">
<div class="card-calc">
<h4>💰 Hitung Beli</h4>
<p class="calc-rate">Harga: <span id="buyRateDisplay">-</span>/gr</p>
<div class="calc-input-group">
<label>Masukkan Rupiah (IDR)</label>
<input type="text" id="buyIdr" inputmode="numeric" placeholder="Contoh: 88.000.000">
</div>
<div class="calc-input-group">
<label>Gram ⇆</label>
<input type="text" id="buyGram" inputmode="decimal" placeholder="isi gram, contoh: 0.8888">
</div>
</div>
<div class="card-calc">
<h4>💸 Hitung Jual</h4>
<p class="calc-rate">Harga: <span id="sellRateDisplay">-</span>/gr</p>
<div class="calc-input-group">
<label>Masukkan Gram</label>
<input type="text" id="sellGram" inputmode="decimal" placeholder="Contoh: 0.8888">
</div>
<div class="calc-input-group">
<label>Rupiah (IDR) ⇆</label>
<input type="text" id="sellIdr" inputmode="numeric" placeholder="isi nominal, contoh: 88.000.000">
</div>
</div>
</div>
</div>
</div>
<footer id="footerApp"><span class="marquee-text">&copy;2026 ~ahmadkholil~</span></footer>
<script src="https://code.jquery.com/jquery-3.7.0.min.js"></script>
<script src="https://cdn.datatables.net/1.13.6/js/jquery.dataTables.min.js"></script>
<script src="https://s3.tradingview.com/tv.js"></script>
<script>
(function(){
var isDark=localStorage.getItem('theme')==='dark';
var lastTopRowId='';
var messageQueue=[];
var isProcessing=false;
var latestHistory=[];
var isFirstRender=true;
var savedPriority=localStorage.getItem('profitPriority');
var profitPriority=(savedPriority&&['jt10','jt20','jt30','jt40','jt50'].indexOf(savedPriority)!==-1)?savedPriority:'jt10';
var headerLabels={'jt10':'Est.cuan 10JT ➺ gr','jt20':'Est.cuan 20JT ➺ gr','jt30':'Est.cuan 30JT ➺ gr','jt40':'Est.cuan 40JT ➺ gr','jt50':'Est.cuan 50JT ➺ gr'};
var blinkTimeout=null;
var latestBuyRate=0;
var latestSellRate=0;
var isCalcUpdating=false;
function parseRupiahInput(str){if(!str)return 0;return parseInt(str.replace(/\./g,''),10)||0}
function formatRupiahCalc(n){if(isNaN(n)||n===0)return'';return Math.round(n).toLocaleString('id-ID').replace(/,/g,'.')}
function formatRupiahLive(str){var num=str.replace(/\D/g,'');if(!num)return'';return parseInt(num,10).toLocaleString('id-ID').replace(/,/g,'.')}
function parseGramInput(str){if(!str)return 0;return parseFloat(str.replace(',','.'))||0}
function formatGramCalc(n){if(isNaN(n)||n===0)return'';return n.toFixed(4).replace('.',',')}
function updateCalcRates(){document.getElementById('buyRateDisplay').textContent=latestBuyRate?formatRupiahCalc(latestBuyRate):'-';document.getElementById('sellRateDisplay').textContent=latestSellRate?formatRupiahCalc(latestSellRate):'-'}
function setupCalcListeners(){
var buyIdr=document.getElementById('buyIdr');var buyGram=document.getElementById('buyGram');var sellIdr=document.getElementById('sellIdr');var sellGram=document.getElementById('sellGram');
function onlyNumbers(e){var key=e.key;if(e.ctrlKey||e.metaKey)return;if(key==='Backspace'||key==='Delete'||key==='ArrowLeft'||key==='ArrowRight'||key==='Tab')return;if(!/^[0-9]$/.test(key)){e.preventDefault()}}
function onlyNumbersAndComma(e){var key=e.key;var val=this.value;if(e.ctrlKey||e.metaKey)return;if(key==='Backspace'||key==='Delete'||key==='ArrowLeft'||key==='ArrowRight'||key==='Tab')return;if(key===','||key==='.'){if(val.indexOf(',')!==-1||val.indexOf('.')!==-1){e.preventDefault();return}return}if(!/^[0-9]$/.test(key)){e.preventDefault()}}
buyIdr.addEventListener('keypress',onlyNumbers);sellIdr.addEventListener('keypress',onlyNumbers);buyGram.addEventListener('keypress',onlyNumbersAndComma);sellGram.addEventListener('keypress',onlyNumbersAndComma);
buyIdr.addEventListener('paste',function(e){e.preventDefault();var text=(e.clipboardData||window.clipboardData).getData('text');var nums=text.replace(/\D/g,'');if(nums){document.execCommand('insertText',false,nums)}});
sellIdr.addEventListener('paste',function(e){e.preventDefault();var text=(e.clipboardData||window.clipboardData).getData('text');var nums=text.replace(/\D/g,'');if(nums){document.execCommand('insertText',false,nums)}});
buyGram.addEventListener('paste',function(e){e.preventDefault();var text=(e.clipboardData||window.clipboardData).getData('text');var clean=text.replace(/[^0-9,.]/g,'').replace(/\./g,',');var parts=clean.split(',');var result=parts[0]+(parts.length>1?','+parts.slice(1).join(''):'');if(result){document.execCommand('insertText',false,result)}});
sellGram.addEventListener('paste',function(e){e.preventDefault();var text=(e.clipboardData||window.clipboardData).getData('text');var clean=text.replace(/[^0-9,.]/g,'').replace(/\./g,',');var parts=clean.split(',');var result=parts[0]+(parts.length>1?','+parts.slice(1).join(''):'');if(result){document.execCommand('insertText',false,result)}});
buyIdr.addEventListener('input',function(){if(isCalcUpdating)return;isCalcUpdating=true;var cursorPos=this.selectionStart;var oldLen=this.value.length;var formatted=formatRupiahLive(this.value);this.value=formatted;var newLen=formatted.length;var newPos=cursorPos+(newLen-oldLen);if(newPos<0)newPos=0;if(newPos>newLen)newPos=newLen;this.setSelectionRange(newPos,newPos);if(latestBuyRate){var val=parseRupiahInput(formatted);var gram=val/latestBuyRate;buyGram.value=gram>0?formatGramCalc(gram):''}isCalcUpdating=false});
buyGram.addEventListener('input',function(){if(isCalcUpdating||!latestBuyRate)return;isCalcUpdating=true;var gram=parseGramInput(this.value);var idr=gram*latestBuyRate;buyIdr.value=idr>0?formatRupiahCalc(idr):'';isCalcUpdating=false});
sellGram.addEventListener('input',function(){if(isCalcUpdating||!latestSellRate)return;isCalcUpdating=true;var gram=parseGramInput(this.value);var idr=gram*latestSellRate;sellIdr.value=idr>0?formatRupiahCalc(idr):'';isCalcUpdating=false});
sellIdr.addEventListener('input',function(){if(isCalcUpdating)return;isCalcUpdating=true;var cursorPos=this.selectionStart;var oldLen=this.value.length;var formatted=formatRupiahLive(this.value);this.value=formatted;var newLen=formatted.length;var newPos=cursorPos+(newLen-oldLen);if(newPos<0)newPos=0;if(newPos>newLen)newPos=newLen;this.setSelectionRange(newPos,newPos);if(latestSellRate){var val=parseRupiahInput(formatted);var gram=val/latestSellRate;sellGram.value=gram>0?formatGramCalc(gram):''}isCalcUpdating=false});
}
setupCalcListeners();
function getOrderedProfitKeys(){var all=['jt10','jt20','jt30','jt40','jt50'];var result=[profitPriority];all.forEach(function(k){if(k!==profitPriority)result.push(k)});return result}
function updateTableHeaders(){var keys=getOrderedProfitKeys();$('#thP1').text(headerLabels[keys[0]]);$('#thP2').text(headerLabels[keys[1]]);$('#thP3').text(headerLabels[keys[2]]);$('#thP4').text(headerLabels[keys[3]]);$('#thP5').text(headerLabels[keys[4]])}
function createTradingViewWidget(){var wrapper=document.getElementById('tradingview_chart');var h=wrapper.offsetHeight||370;new TradingView.widget({width:"100%",height:h,symbol:"OANDA:XAUUSD",interval:"15",timezone:"Asia/Jakarta",theme:isDark?'dark':'light',style:"1",locale:"id",toolbar_bg:"#f1f3f6",enable_publishing:false,hide_top_toolbar:false,save_image:false,container_id:"tradingview_chart"})}
var table=$('#tabel').DataTable({pageLength:4,lengthMenu:[4,8,18,48,88,888,1441],order:[],deferRender:true,dom:'<"dt-top-controls"lf>t<"bottom"p><"clear">',columns:[{data:"waktu",className:"waktu"},{data:"transaction",className:"transaksi"},{data:"p1",className:"profit"},{data:"p2",className:"profit"},{data:"p3",className:"profit"},{data:"p4",className:"profit"},{data:"p5",className:"profit"}],language:{emptyTable:"Menunggu data harga emas dari Treasury...",zeroRecords:"Tidak ada data yang cocok",lengthMenu:"Lihat _MENU_",search:"Cari:",paginate:{first:"«",previous:"Kembali",next:"Lanjut",last:"»"}},initComplete:function(){var filterDiv=$('.dataTables_filter');var activeVal=profitPriority.replace('jt','');var profitBtns=$('<div class="profit-order-btns" id="profitOrderBtns"><button class="profit-btn'+(activeVal==='10'?' active':'')+'" data-val="10">10</button><button class="profit-btn'+(activeVal==='20'?' active':'')+'" data-val="20">20</button><button class="profit-btn'+(activeVal==='30'?' active':'')+'" data-val="30">30</button><button class="profit-btn'+(activeVal==='40'?' active':'')+'" data-val="40">40</button><button class="profit-btn'+(activeVal==='50'?' active':'')+'" data-val="50">50</button></div>');filterDiv.wrap('<div class="filter-wrap"></div>');filterDiv.before(profitBtns);$('#profitOrderBtns').on('click','.profit-btn',function(){var val=$(this).data('val');profitPriority='jt'+val;localStorage.setItem('profitPriority',profitPriority);$('#profitOrderBtns .profit-btn').removeClass('active');$(this).addClass('active');if(latestHistory.length){renderTable()}});updateTableHeaders()}});
function getTopRowId(h){if(!h||!h.length)return'';var sorted=h.slice().sort(function(a,b){return new Date(b.created_at)-new Date(a.created_at)});return sorted[0].created_at+'|'+sorted[0].buying_rate}
function triggerBlinkEffect(){if(blinkTimeout){clearTimeout(blinkTimeout)}var firstRow=$('#tabel tbody tr:first-child');if(!firstRow.length)return;firstRow.removeClass('blink-row');void firstRow[0].offsetWidth;firstRow.addClass('blink-row');blinkTimeout=setTimeout(function(){firstRow.removeClass('blink-row');blinkTimeout=null},2000)}
function renderTable(){var h=latestHistory;if(!h||!h.length)return;var newTopRowId=getTopRowId(h);var isNewData=newTopRowId!==lastTopRowId;if(isNewData){lastTopRowId=newTopRowId}h.sort(function(a,b){return new Date(b.created_at)-new Date(a.created_at)});var keys=getOrderedProfitKeys();updateTableHeaders();var arr=h.map(function(d){return{waktu:d.waktu_display,transaction:'Beli: '+d.buying_rate+' Jual: '+d.selling_rate+''+d.diff_display,p1:d[keys[0]],p2:d[keys[1]],p3:d[keys[2]],p4:d[keys[3]],p5:d[keys[4]]}});table.clear().rows.add(arr).draw(false);table.page('first').draw(false);if(isNewData&&!isFirstRender){setTimeout(function(){triggerBlinkEffect()},50)}if(isFirstRender){isFirstRender=false}}
function updateTable(h){if(!h||!h.length)return;latestHistory=h;h.sort(function(a,b){return new Date(b.created_at)-new Date(a.created_at)});if(h.length>0&&h[0].buying_rate_raw&&h[0].selling_rate_raw){latestBuyRate=h[0].buying_rate_raw;latestSellRate=h[0].selling_rate_raw;updateCalcRates()}renderTable()}
function updateUsd(h){var c=document.getElementById("currentPrice"),p=document.getElementById("priceList");if(!h||!h.length){c.textContent="Menunggu data...";c.className="loading-text";p.innerHTML='<li class="loading-text">Menunggu data...</li>';return}c.className="";function prs(s){return parseFloat(s.trim().replace(/\./g,'').replace(',','.'))}var r=h.slice().reverse();var icon="➖";if(r.length>1){var n=prs(r[0].price),pr=prs(r[1].price);icon=n>pr?"🚀":n<pr?"🔻":"➖"}c.innerHTML=r[0].price+" "+icon;var html='';for(var i=0;i<r.length;i++){var ic="➖";if(i===0&&r.length>1){var n=prs(r[0].price),pr=prs(r[1].price);ic=n>pr?"🟢":n<pr?"🔴":"➖"}else if(i<r.length-1){var n=prs(r[i].price),nx=prs(r[i+1].price);ic=n>nx?"🟢":n<nx?"🔴":"➖"}else if(r.length>1){var n=prs(r[i].price),pr=prs(r[i-1].price);ic=n<pr?"🔴":n>pr?"🟢":"➖"}html+='<li>'+r[i].price+' <span class="time">('+r[i].time+')</span> '+ic+'</li>'}p.innerHTML=html}
function updateLimit(val){document.getElementById('limitBulan').textContent=val}
function processMessage(d){if(d.ping)return;if(d.history)updateTable(d.history);if(d.usd_idr_history)updateUsd(d.usd_idr_history);if(d.limit_bulan!==undefined)updateLimit(d.limit_bulan)}
function processQueue(){if(isProcessing||!messageQueue.length)return;isProcessing=true;var msg=messageQueue.shift();try{processMessage(msg)}catch(e){}isProcessing=false;if(messageQueue.length)requestAnimationFrame(processQueue)}
var ws,ra=0,pingInterval;
function conn(){var pr=location.protocol==="https:"?"wss:":"ws:";ws=new WebSocket(pr+"//"+location.host+"/ws");ws.binaryType='arraybuffer';ws.onopen=function(){ra=0;if(pingInterval)clearInterval(pingInterval);pingInterval=setInterval(function(){if(ws&&ws.readyState===1)try{ws.send('ping')}catch(e){}},25000)};ws.onmessage=function(e){try{var d;if(e.data instanceof ArrayBuffer){d=JSON.parse(new TextDecoder().decode(e.data))}else{d=JSON.parse(e.data)}messageQueue.push(d);requestAnimationFrame(processQueue)}catch(x){}};ws.onclose=function(){if(pingInterval)clearInterval(pingInterval);ra++;setTimeout(conn,Math.min(1000*Math.pow(1.3,ra-1),15000))};ws.onerror=function(){}}
conn();
function updateJam(){var n=new Date();var days=['Minggu','Senin','Selasa','Rabu','Kamis','Jumat','Sabtu'];var hari=days[n.getDay()];var tgl=n.toLocaleDateString('id-ID',{day:'2-digit',month:'long',year:'numeric'});var jam=n.toLocaleTimeString('id-ID',{hour12:false});document.getElementById("jam").textContent=hari+", "+jam+" WIB"}
setInterval(updateJam,1000);updateJam();
window.toggleTheme=function(){var b=document.body,btn=document.getElementById('themeBtn');b.classList.toggle('dark-mode');isDark=b.classList.contains('dark-mode');btn.textContent=isDark?"☀️":"🌙";localStorage.setItem('theme',isDark?'dark':'light');document.getElementById('tradingview_chart').innerHTML='';createTradingViewWidget()};
if(localStorage.getItem('theme')==='dark'){document.body.classList.add('dark-mode');document.getElementById('themeBtn').textContent="☀️"}
setTimeout(createTradingViewWidget,100);
})();
</script>
</body>
</html>"##;
