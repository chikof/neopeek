use super::{Ascii, SPACE};

pub const CAT_ASCII: [&str; 36] = [
SPACE, SPACE,
"\x1b[49m              \x1b[48;2;0;0;0m          \x1b[49m                                                \x1b[48;2;0;0;0m    \x1b[48;2;1;1;1m  \x1b[48;2;0;0;0m  \x1b[49m                \x1b[m",
"\x1b[49m            \x1b[48;2;0;0;0m  \x1b[48;2;28;24;23m  \x1b[48;2;140;119;112m  \x1b[48;2;131;112;105m  \x1b[48;2;13;12;10m  \x1b[48;2;0;0;0m    \x1b[49m                                          \x1b[48;2;0;0;0m    \x1b[48;2;6;5;5m  \x1b[48;2;130;110;104m  \x1b[48;2;127;108;102m  \x1b[48;2;0;0;0m    \x1b[49m              \x1b[m",
"\x1b[49m            \x1b[48;2;0;0;0m  \x1b[48;2;39;32;31m  \x1b[48;2;170;144;136m      \x1b[48;2;100;85;80m  \x1b[48;2;4;4;4m  \x1b[48;2;0;0;0m  \x1b[49m                                      \x1b[48;2;0;0;0m  \x1b[48;2;1;1;1m  \x1b[48;2;97;97;96m  \x1b[48;2;158;135;128m  \x1b[48;2;170;144;136m    \x1b[48;2;0;0;0m  \x1b[48;2;0;1;0m  \x1b[49m              \x1b[m",
"\x1b[49m            \x1b[48;2;0;0;0m  \x1b[48;2;62;53;50m  \x1b[48;2;170;144;136m      \x1b[48;2;196;178;173m  \x1b[48;2;241;241;241m  \x1b[48;2;25;25;25m  \x1b[48;2;0;0;0m  \x1b[49m                                  \x1b[48;2;0;0;0m    \x1b[48;2;119;119;119m  \x1b[48;2;255;255;255m  \x1b[48;2;247;245;244m  \x1b[48;2;173;147;139m  \x1b[48;2;170;144;136m  \x1b[48;2;14;11;11m  \x1b[48;2;0;1;0m  \x1b[49m              \x1b[m",
"\x1b[49m            \x1b[48;2;0;0;0m  \x1b[48;2;109;93;88m  \x1b[48;2;190;170;163m  \x1b[48;2;198;185;181m  \x1b[48;2;178;169;166m  \x1b[48;2;255;254;254m  \x1b[48;2;255;255;255m  \x1b[48;2;217;217;217m  \x1b[48;2;68;68;68m  \x1b[48;2;0;0;0m  \x1b[49m                              \x1b[48;2;0;0;0m  \x1b[48;2;7;7;6m  \x1b[48;2;138;138;138m  \x1b[48;2;255;255;255m    \x1b[48;2;207;207;207m  \x1b[48;2;162;158;155m  \x1b[48;2;225;215;213m  \x1b[48;2;73;70;69m  \x1b[48;2;0;0;0m  \x1b[49m              \x1b[m",
"\x1b[49m            \x1b[48;2;0;0;0m  \x1b[48;2;216;216;216m  \x1b[48;2;255;255;255m  \x1b[48;2;255;254;254m  \x1b[48;2;64;60;57m  \x1b[48;2;177;176;176m  \x1b[48;2;255;255;255m    \x1b[48;2;240;240;240m  \x1b[48;2;53;53;53m  \x1b[48;2;0;1;0m  \x1b[48;2;0;0;0m    \x1b[48;2;0;1;0m  \x1b[48;2;0;0;0m  \x1b[48;2;0;1;1m  \x1b[48;2;3;2;3m  \x1b[48;2;4;4;5m  \x1b[48;2;1;1;1m  \x1b[48;2;0;0;0m  \x1b[48;2;0;0;1m  \x1b[48;2;0;0;0m  \x1b[48;2;0;1;0m  \x1b[49m  \x1b[48;2;0;0;0m    \x1b[48;2;106;106;106m  \x1b[48;2;255;255;255m    \x1b[48;2;231;231;231m  \x1b[48;2;124;112;105m  \x1b[48;2;255;231;218m  \x1b[48;2;255;255;254m  \x1b[48;2;149;148;148m  \x1b[48;2;0;0;0m  \x1b[49m              \x1b[m",
"\x1b[49m          \x1b[48;2;0;0;0m    \x1b[48;2;245;245;245m  \x1b[48;2;255;255;255m  \x1b[48;2;255;252;251m  \x1b[48;2;255;230;217m  \x1b[48;2;133;121;115m  \x1b[48;2;173;172;172m  \x1b[48;2;255;255;255m    \x1b[48;2;220;220;220m  \x1b[48;2;21;21;21m  \x1b[48;2;0;0;0m  \x1b[48;2;45;45;45m  \x1b[48;2;152;152;152m  \x1b[48;2;167;167;167m  \x1b[48;2;170;170;170m  \x1b[48;2;178;178;178m  \x1b[48;2;180;180;180m  \x1b[48;2;172;172;172m  \x1b[48;2;167;167;167m  \x1b[48;2;153;153;153m  \x1b[48;2;62;62;62m  \x1b[48;2;0;0;0m      \x1b[48;2;115;115;115m  \x1b[48;2;255;255;254m  \x1b[48;2;254;255;255m  \x1b[48;2;190;190;190m  \x1b[48;2;152;137;129m  \x1b[48;2;251;225;211m  \x1b[48;2;255;230;218m  \x1b[48;2;255;255;255m  \x1b[48;2;184;184;184m  \x1b[48;2;0;0;0m  \x1b[49m              \x1b[m",
"\x1b[49m          \x1b[48;2;0;0;1m  \x1b[48;2;0;0;0m  \x1b[48;2;254;254;254m  \x1b[48;2;255;255;255m  \x1b[48;2;255;252;251m  \x1b[48;2;255;230;217m  \x1b[48;2;255;228;214m  \x1b[48;2;81;74;70m  \x1b[48;2;212;212;212m  \x1b[48;2;255;255;255m    \x1b[48;2;225;225;225m  \x1b[48;2;235;235;235m  \x1b[48;2;255;255;255m                    \x1b[48;2;254;254;254m  \x1b[48;2;166;166;166m  \x1b[48;2;73;73;73m  \x1b[48;2;230;230;230m  \x1b[48;2;255;255;255m  \x1b[48;2;235;235;235m  \x1b[48;2;121;111;105m  \x1b[48;2;254;227;212m  \x1b[48;2;255;228;214m  \x1b[48;2;255;232;219m  \x1b[48;2;255;255;255m  \x1b[48;2;198;198;198m  \x1b[48;2;0;0;0m  \x1b[49m              \x1b[m",
"\x1b[49m          \x1b[48;2;0;0;0m    \x1b[48;2;255;255;255m    \x1b[48;2;255;254;253m  \x1b[48;2;255;230;217m  \x1b[48;2;255;228;214m  \x1b[48;2;252;225;212m  \x1b[48;2;113;107;103m  \x1b[48;2;255;255;255m            \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m                \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m      \x1b[48;2;82;82;82m  \x1b[48;2;236;211;198m  \x1b[48;2;255;228;214m    \x1b[48;2;255;236;225m  \x1b[48;2;255;255;254m  \x1b[48;2;199;199;199m  \x1b[48;2;0;0;0m  \x1b[49m              \x1b[m",
"\x1b[49m          \x1b[48;2;0;0;0m    \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m    \x1b[48;2;255;230;218m  \x1b[48;2;255;228;214m  \x1b[48;2;254;228;214m  \x1b[48;2;184;172;165m  \x1b[48;2;239;239;239m  \x1b[48;2;255;255;255m                    \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m    \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m        \x1b[48;2;254;255;255m  \x1b[48;2;255;247;242m  \x1b[48;2;255;234;224m  \x1b[48;2;255;228;215m  \x1b[48;2;255;249;245m  \x1b[48;2;255;255;255m  \x1b[48;2;199;199;199m  \x1b[48;2;0;0;0m  \x1b[49m              \x1b[m",
"\x1b[49m          \x1b[48;2;0;0;0m  \x1b[48;2;1;0;0m  \x1b[48;2;255;255;255m      \x1b[48;2;255;244;239m  \x1b[48;2;255;234;224m  \x1b[48;2;255;252;250m  \x1b[48;2;255;254;254m  \x1b[48;2;255;255;255m                              \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m  \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m      \x1b[48;2;255;254;254m  \x1b[48;2;255;253;251m  \x1b[48;2;255;255;255m    \x1b[48;2;198;199;199m  \x1b[48;2;0;0;0m  \x1b[49m              \x1b[m",
"\x1b[49m          \x1b[48;2;0;0;1m  \x1b[48;2;2;2;2m  \x1b[48;2;255;255;255m  \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m            \x1b[48;2;254;255;255m  \x1b[48;2;255;255;255m            \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m  \x1b[48;2;254;255;255m  \x1b[48;2;255;255;255m          \x1b[48;2;254;255;255m  \x1b[48;2;255;255;255m                \x1b[48;2;199;199;199m  \x1b[48;2;0;0;0m  \x1b[49m              \x1b[m",
"\x1b[49m          \x1b[48;2;0;0;0m  \x1b[48;2;5;5;5m  \x1b[48;2;255;255;255m  \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m                      \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m                        \x1b[48;2;254;255;255m  \x1b[48;2;255;255;255m          \x1b[48;2;199;199;199m  \x1b[48;2;0;0;0m  \x1b[49m              \x1b[m",
"\x1b[49m          \x1b[48;2;0;0;0m  \x1b[48;2;9;9;9m  \x1b[48;2;255;255;255m    \x1b[48;2;254;255;255m  \x1b[48;2;255;255;255m        \x1b[48;2;255;255;254m  \x1b[48;2;254;255;255m  \x1b[48;2;255;255;255m                                          \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m  \x1b[48;2;199;199;199m  \x1b[48;2;0;0;0m  \x1b[49m              \x1b[m",
"\x1b[49m          \x1b[48;2;0;0;0m  \x1b[48;2;66;66;66m  \x1b[48;2;255;255;255m      \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m                \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m                        \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m            \x1b[48;2;204;204;204m  \x1b[48;2;5;5;5m  \x1b[49m              \x1b[m",
"\x1b[49m          \x1b[48;2;1;1;1m  \x1b[48;2;171;171;171m  \x1b[48;2;255;255;255m              \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m                \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m                              \x1b[48;2;247;247;247m  \x1b[48;2;48;48;48m  \x1b[48;2;0;0;0m  \x1b[49m            \x1b[m",
"\x1b[49m        \x1b[48;2;0;0;0m  \x1b[48;2;19;19;19m  \x1b[48;2;235;235;235m  \x1b[48;2;255;255;255m      \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m        \x1b[48;2;254;255;255m  \x1b[48;2;255;255;255m  \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m                      \x1b[48;2;255;255;254m  \x1b[48;2;254;255;255m  \x1b[48;2;255;255;255m    \x1b[48;2;254;255;255m  \x1b[48;2;255;255;255m          \x1b[48;2;254;255;255m  \x1b[48;2;111;110;111m  \x1b[48;2;0;0;0m  \x1b[49m            \x1b[m",
"\x1b[49m        \x1b[48;2;0;0;0m  \x1b[48;2;27;27;27m  \x1b[48;2;255;255;255m  \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m      \x1b[48;2;255;255;254m  \x1b[48;2;254;255;255m  \x1b[48;2;255;255;255m      \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m        \x1b[48;2;254;255;255m  \x1b[48;2;255;255;255m  \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m                \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m              \x1b[48;2;245;245;245m  \x1b[48;2;0;0;0m    \x1b[49m          \x1b[m",
"\x1b[49m        \x1b[48;2;0;0;0m  \x1b[48;2;109;109;109m  \x1b[48;2;255;255;255m          \x1b[48;2;255;255;254m  \x1b[48;2;254;255;255m  \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m                    \x1b[48;2;254;255;255m  \x1b[48;2;255;255;255m      \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m  \x1b[48;2;254;255;255m  \x1b[48;2;255;255;255m                    \x1b[48;2;12;12;12m  \x1b[48;2;0;0;0m  \x1b[49m  \x1b[48;2;0;0;0m      \x1b[49m  \x1b[m",
"\x1b[49m        \x1b[48;2;0;0;0m  \x1b[48;2;203;203;203m  \x1b[48;2;255;255;255m      \x1b[48;2;254;255;255m  \x1b[48;2;255;255;255m  \x1b[48;2;200;200;200m  \x1b[48;2;9;8;8m  \x1b[48;2;13;12;13m  \x1b[48;2;234;234;234m  \x1b[48;2;254;255;255m  \x1b[48;2;255;255;255m                              \x1b[48;2;107;107;107m  \x1b[48;2;8;8;8m  \x1b[48;2;48;48;48m  \x1b[48;2;255;255;255m      \x1b[48;2;255;254;255m  \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m    \x1b[48;2;70;70;70m  \x1b[48;2;0;0;0m    \x1b[49m        \x1b[m",
"\x1b[49m        \x1b[48;2;0;0;0m  \x1b[48;2;231;231;231m  \x1b[48;2;255;255;255m          \x1b[48;2;70;70;70m  \x1b[48;2;210;210;210m  \x1b[48;2;198;198;198m  \x1b[48;2;30;30;30m  \x1b[48;2;230;230;230m  \x1b[48;2;255;255;255m          \x1b[48;2;254;255;255m  \x1b[48;2;255;255;255m              \x1b[48;2;255;255;254m  \x1b[48;2;206;206;206m  \x1b[48;2;101;101;101m  \x1b[48;2;236;236;236m  \x1b[48;2;150;150;150m  \x1b[48;2;106;107;106m  \x1b[48;2;255;255;255m        \x1b[48;2;240;240;240m  \x1b[48;2;176;176;176m  \x1b[48;2;124;124;124m  \x1b[48;2;1;0;0m  \x1b[48;2;0;0;0m        \x1b[48;2;0;0;1m  \x1b[m",
"\x1b[49m        \x1b[48;2;0;0;0m  \x1b[48;2;221;221;220m  \x1b[48;2;253;253;253m  \x1b[48;2;255;255;255m  \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m    \x1b[48;2;225;225;224m  \x1b[48;2;255;255;255m    \x1b[48;2;200;200;200m  \x1b[48;2;240;241;241m  \x1b[48;2;255;255;255m  \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m  \x1b[48;2;255;254;255m  \x1b[48;2;215;215;215m  \x1b[48;2;254;254;254m  \x1b[48;2;84;84;84m  \x1b[48;2;239;239;239m  \x1b[48;2;236;236;236m  \x1b[48;2;80;81;80m  \x1b[48;2;252;252;252m  \x1b[48;2;255;255;255m  \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m  \x1b[48;2;234;235;235m  \x1b[48;2;221;221;221m  \x1b[48;2;255;255;255m    \x1b[48;2;216;216;216m  \x1b[48;2;255;248;249m  \x1b[48;2;254;246;246m  \x1b[48;2;255;246;246m  \x1b[48;2;255;255;255m    \x1b[48;2;87;87;87m  \x1b[48;2;27;27;27m  \x1b[48;2;0;0;0m  \x1b[49m          \x1b[m",
"\x1b[48;2;0;0;0m    \x1b[48;2;0;0;1m  \x1b[48;2;1;0;0m  \x1b[48;2;0;0;0m  \x1b[48;2;117;117;117m  \x1b[48;2;240;240;240m  \x1b[48;2;255;255;255m      \x1b[48;2;255;238;239m  \x1b[48;2;255;213;213m  \x1b[48;2;255;217;217m  \x1b[48;2;255;248;248m  \x1b[48;2;255;255;255m    \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m      \x1b[48;2;142;142;142m  \x1b[48;2;84;84;84m  \x1b[48;2;127;127;127m  \x1b[48;2;90;90;90m  \x1b[48;2;91;91;91m  \x1b[48;2;206;206;206m  \x1b[48;2;255;255;255m    \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m      \x1b[48;2;255;255;254m  \x1b[48;2;254;255;255m  \x1b[48;2;255;194;194m  \x1b[48;2;255;166;166m  \x1b[48;2;255;156;156m  \x1b[48;2;255;165;164m  \x1b[48;2;255;250;250m  \x1b[48;2;255;255;255m    \x1b[48;2;141;141;141m  \x1b[48;2;0;0;0m  \x1b[49m          \x1b[m",
"\x1b[49m        \x1b[48;2;0;0;0m  \x1b[48;2;25;25;24m  \x1b[48;2;220;220;220m  \x1b[48;2;255;255;255m    \x1b[48;2;255;244;244m  \x1b[48;2;255;166;166m  \x1b[48;2;255;156;156m      \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m    \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m                          \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m  \x1b[48;2;255;189;189m  \x1b[48;2;255;156;156m    \x1b[48;2;255;168;168m  \x1b[48;2;255;195;195m  \x1b[48;2;255;255;255m    \x1b[48;2;130;131;130m  \x1b[48;2;9;9;9m  \x1b[48;2;0;0;0m      \x1b[48;2;1;0;0m  \x1b[48;2;0;1;0m  \x1b[49m  \x1b[m",
"\x1b[49m  \x1b[48;2;0;0;0m    \x1b[48;2;1;0;0m  \x1b[48;2;0;0;0m  \x1b[48;2;5;5;5m  \x1b[48;2;167;167;167m  \x1b[48;2;255;255;255m      \x1b[48;2;255;239;239m  \x1b[48;2;255;230;230m  \x1b[48;2;255;204;204m  \x1b[48;2;255;217;217m  \x1b[48;2;255;255;255m    \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m  \x1b[48;2;254;255;255m  \x1b[48;2;255;255;255m  \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m        \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m  \x1b[48;2;254;255;255m  \x1b[48;2;255;255;255m          \x1b[48;2;255;232;232m  \x1b[48;2;255;228;228m  \x1b[48;2;255;233;232m  \x1b[48;2;255;246;246m  \x1b[48;2;255;255;255m      \x1b[48;2;242;243;242m  \x1b[48;2;1;1;0m  \x1b[48;2;0;0;0m  \x1b[49m          \x1b[m",
"\x1b[48;2;1;0;0m  \x1b[48;2;0;0;0m  \x1b[49m    \x1b[48;2;0;0;0m    \x1b[48;2;17;17;17m  \x1b[48;2;200;200;200m  \x1b[48;2;255;255;255m                    \x1b[48;2;254;255;255m  \x1b[48;2;255;255;255m  \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m                    \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m    \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m        \x1b[48;2;244;244;244m  \x1b[48;2;54;55;54m  \x1b[48;2;0;0;0m  \x1b[49m            \x1b[m",
"\x1b[49m      \x1b[48;2;0;0;0m        \x1b[48;2;79;79;79m  \x1b[48;2;255;255;255m                    \x1b[48;2;254;255;255m  \x1b[48;2;255;255;255m                  \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m        \x1b[48;2;254;255;255m  \x1b[48;2;255;255;255m          \x1b[48;2;127;127;127m  \x1b[48;2;2;3;2m  \x1b[49m              \x1b[m",
"\x1b[49m    \x1b[48;2;0;0;0m  \x1b[49m      \x1b[48;2;0;0;0m    \x1b[48;2;97;97;97m  \x1b[48;2;211;211;211m  \x1b[48;2;255;255;255m                    \x1b[48;2;255;255;254m  \x1b[48;2;254;255;255m  \x1b[48;2;255;255;255m                    \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m      \x1b[48;2;254;255;255m  \x1b[48;2;237;237;237m  \x1b[48;2;82;82;82m  \x1b[48;2;0;0;0m    \x1b[49m              \x1b[m",
"\x1b[49m                \x1b[48;2;0;0;0m    \x1b[48;2;143;143;143m  \x1b[48;2;248;248;248m  \x1b[48;2;254;255;255m  \x1b[48;2;255;255;255m            \x1b[48;2;255;254;255m  \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m        \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m                \x1b[48;2;254;254;254m  \x1b[48;2;248;248;248m  \x1b[48;2;140;140;140m  \x1b[48;2;0;0;0m    \x1b[49m                  \x1b[m",
"\x1b[49m                  \x1b[48;2;1;0;0m  \x1b[48;2;0;0;0m  \x1b[48;2;18;18;18m  \x1b[48;2;99;99;99m  \x1b[48;2;135;135;135m  \x1b[48;2;234;234;234m  \x1b[48;2;255;255;255m  \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m  \x1b[48;2;254;255;255m  \x1b[48;2;255;255;255m    \x1b[48;2;254;255;255m  \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m                  \x1b[48;2;238;238;238m  \x1b[48;2;136;136;136m  \x1b[48;2;94;95;94m  \x1b[48;2;12;12;12m  \x1b[48;2;0;0;0m    \x1b[49m                    \x1b[m",
"\x1b[49m                      \x1b[48;2;0;0;0m        \x1b[48;2;1;1;1m  \x1b[48;2;55;55;55m  \x1b[48;2;156;156;156m  \x1b[48;2;208;208;208m  \x1b[48;2;215;215;215m    \x1b[48;2;216;216;216m  \x1b[48;2;218;218;218m  \x1b[48;2;219;219;218m  \x1b[48;2;218;218;218m  \x1b[48;2;217;217;217m  \x1b[48;2;215;215;215m    \x1b[48;2;213;213;213m  \x1b[48;2;166;166;166m  \x1b[48;2;68;68;68m  \x1b[48;2;5;5;5m  \x1b[48;2;0;0;1m  \x1b[48;2;0;0;0m      \x1b[49m                        \x1b[m",
"\x1b[49m                            \x1b[48;2;0;0;0m              \x1b[48;2;2;2;2m  \x1b[48;2;5;5;5m  \x1b[48;2;7;7;7m  \x1b[48;2;6;6;6m  \x1b[48;2;3;2;3m  \x1b[48;2;0;0;0m  \x1b[48;2;1;0;0m  \x1b[48;2;0;0;0m          \x1b[49m                              \x1b[m",
SPACE, SPACE
];

pub const CAT_ASCII_MINI: [&str; 34] = [
SPACE, SPACE, SPACE, SPACE, SPACE, SPACE,
"\x1b[49m        \x1b[48;2;0;1;0m  \x1b[48;2;11;9;9m  \x1b[48;2;11;10;9m  \x1b[48;2;14;11;11m  \x1b[49m                                \x1b[48;2;0;0;0m  \x1b[48;2;11;9;9m  \x1b[48;2;11;9;8m  \x1b[48;2;0;0;0m  \x1b[49m          \x1b[m",
"\x1b[49m        \x1b[48;2;0;1;0m  \x1b[48;2;141;120;113m  \x1b[48;2;171;144;136m  \x1b[48;2;90;77;72m  \x1b[48;2;36;33;31m  \x1b[48;2;0;0;0m  \x1b[49m                        \x1b[48;2;0;0;0m  \x1b[48;2;19;19;19m  \x1b[48;2;45;45;45m  \x1b[48;2;154;131;124m  \x1b[48;2;167;142;135m  \x1b[48;2;0;0;0m  \x1b[49m          \x1b[m",
"\x1b[49m        \x1b[48;2;0;0;0m  \x1b[48;2;141;120;113m  \x1b[48;2;174;150;142m  \x1b[48;2;201;185;179m  \x1b[48;2;176;167;165m  \x1b[48;2;66;66;66m  \x1b[48;2;0;0;0m  \x1b[49m                    \x1b[48;2;0;0;0m  \x1b[48;2;40;40;40m  \x1b[48;2;108;108;109m  \x1b[48;2;224;224;224m  \x1b[48;2;160;147;141m  \x1b[48;2;193;176;170m  \x1b[48;2;0;0;0m  \x1b[49m          \x1b[m",
"\x1b[49m        \x1b[48;2;0;0;0m  \x1b[48;2;175;164;161m  \x1b[48;2;217;206;202m  \x1b[48;2;194;189;187m  \x1b[48;2;255;255;255m  \x1b[48;2;197;197;197m  \x1b[48;2;66;66;66m  \x1b[48;2;0;0;0m  \x1b[48;2;1;0;0m  \x1b[48;2;0;1;0m  \x1b[48;2;0;0;1m  \x1b[48;2;1;0;0m  \x1b[48;2;0;1;0m  \x1b[48;2;0;0;1m  \x1b[48;2;0;0;0m  \x1b[48;2;0;1;0m  \x1b[49m  \x1b[48;2;25;25;25m  \x1b[48;2;119;119;119m  \x1b[48;2;236;236;237m  \x1b[48;2;171;171;171m  \x1b[48;2;168;157;151m  \x1b[48;2;247;246;247m  \x1b[48;2;1;0;0m  \x1b[49m          \x1b[m",
"\x1b[49m        \x1b[48;2;0;1;0m  \x1b[48;2;212;212;212m  \x1b[48;2;255;255;255m  \x1b[48;2;196;177;167m  \x1b[48;2;208;201;199m  \x1b[48;2;255;255;255m  \x1b[48;2;197;197;197m  \x1b[48;2;55;55;55m  \x1b[48;2;156;156;156m      \x1b[48;2;157;156;156m  \x1b[48;2;156;157;156m  \x1b[48;2;156;156;157m  \x1b[48;2;157;157;157m  \x1b[48;2;141;140;141m  \x1b[48;2;0;0;0m  \x1b[48;2;55;54;54m  \x1b[48;2;223;223;223m  \x1b[48;2;168;168;168m  \x1b[48;2;142;130;124m  \x1b[48;2;242;216;203m  \x1b[48;2;251;251;251m  \x1b[48;2;1;0;0m  \x1b[49m          \x1b[m",
"\x1b[49m        \x1b[48;2;15;15;15m  \x1b[48;2;215;215;215m  \x1b[48;2;254;255;255m  \x1b[48;2;251;225;211m  \x1b[48;2;182;165;156m  \x1b[48;2;151;150;150m  \x1b[48;2;255;254;255m  \x1b[48;2;208;208;208m  \x1b[48;2;244;245;245m  \x1b[48;2;255;255;255m  \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m      \x1b[48;2;254;255;255m  \x1b[48;2;233;233;233m  \x1b[48;2;191;191;191m  \x1b[48;2;209;208;208m  \x1b[48;2;199;199;199m  \x1b[48;2;159;148;142m  \x1b[48;2;237;212;199m  \x1b[48;2;255;228;214m  \x1b[48;2;251;251;250m  \x1b[48;2;8;8;8m  \x1b[49m          \x1b[m",
"\x1b[49m        \x1b[48;2;61;61;61m  \x1b[48;2;222;222;223m  \x1b[48;2;255;255;255m  \x1b[48;2;255;229;214m  \x1b[48;2;247;221;206m  \x1b[48;2;239;226;219m  \x1b[48;2;255;255;255m        \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m    \x1b[48;2;255;255;254m  \x1b[48;2;254;255;255m  \x1b[48;2;255;255;255m  \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m  \x1b[48;2;246;246;246m  \x1b[48;2;246;242;240m  \x1b[48;2;255;234;223m  \x1b[48;2;255;234;224m  \x1b[48;2;252;252;252m  \x1b[48;2;43;43;43m  \x1b[49m          \x1b[m",
"\x1b[49m        \x1b[48;2;119;119;119m  \x1b[48;2;232;232;232m  \x1b[48;2;255;255;255m  \x1b[48;2;255;254;255m  \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m        \x1b[48;2;255;254;255m  \x1b[48;2;255;255;254m  \x1b[48;2;254;255;255m  \x1b[48;2;255;254;255m  \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m  \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m  \x1b[48;2;254;255;255m  \x1b[48;2;255;255;255m  \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m    \x1b[48;2;252;252;252m  \x1b[48;2;84;84;84m  \x1b[49m          \x1b[m",
"\x1b[49m        \x1b[48;2;160;160;160m  \x1b[48;2;240;240;241m  \x1b[48;2;255;255;255m    \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m    \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m  \x1b[48;2;255;254;255m  \x1b[48;2;255;255;254m  \x1b[48;2;254;255;255m  \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m  \x1b[48;2;254;255;255m  \x1b[48;2;255;255;255m              \x1b[48;2;253;253;253m  \x1b[48;2;96;96;96m  \x1b[49m          \x1b[m",
"\x1b[49m      \x1b[48;2;0;0;0m  \x1b[48;2;176;177;176m  \x1b[48;2;247;247;247m  \x1b[48;2;255;255;255m                      \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m    \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m  \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m    \x1b[48;2;255;254;255m  \x1b[48;2;253;253;252m  \x1b[48;2;115;115;115m  \x1b[48;2;0;0;0m  \x1b[49m        \x1b[m",
"\x1b[49m      \x1b[48;2;0;0;0m  \x1b[48;2;174;175;174m  \x1b[48;2;255;255;255m      \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m  \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m                  \x1b[48;2;255;255;254m  \x1b[48;2;254;255;255m  \x1b[48;2;255;255;255m    \x1b[48;2;254;255;255m  \x1b[48;2;255;255;255m  \x1b[48;2;254;254;254m  \x1b[48;2;190;190;190m  \x1b[48;2;0;0;0m  \x1b[49m        \x1b[m",
"\x1b[49m      \x1b[48;2;36;37;37m  \x1b[48;2;186;186;186m  \x1b[48;2;255;255;255m    \x1b[48;2;255;254;255m  \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m  \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m    \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m        \x1b[48;2;254;255;255m  \x1b[48;2;255;255;255m        \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m      \x1b[48;2;212;212;212m  \x1b[48;2;0;0;0m  \x1b[49m        \x1b[m",
"\x1b[49m    \x1b[48;2;0;0;0m  \x1b[48;2;134;134;134m  \x1b[48;2;237;237;237m  \x1b[48;2;255;255;255m    \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m                  \x1b[48;2;255;255;254m  \x1b[48;2;254;255;255m  \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m      \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m  \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m  \x1b[48;2;235;235;235m  \x1b[48;2;125;124;125m  \x1b[48;2;0;0;1m  \x1b[49m  \x1b[48;2;0;0;0m  \x1b[49m  \x1b[m",
"\x1b[49m    \x1b[48;2;0;0;0m  \x1b[48;2;137;137;137m  \x1b[48;2;255;255;255m    \x1b[48;2;254;255;255m  \x1b[48;2;226;226;226m  \x1b[48;2;44;44;44m  \x1b[48;2;88;88;88m  \x1b[48;2;206;206;206m  \x1b[48;2;255;255;255m    \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m      \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m    \x1b[48;2;252;252;253m  \x1b[48;2;178;178;178m  \x1b[48;2;42;42;42m  \x1b[48;2;161;161;161m  \x1b[48;2;255;255;255m      \x1b[48;2;221;221;221m  \x1b[48;2;41;40;41m  \x1b[48;2;0;0;0m  \x1b[49m  \x1b[48;2;0;0;0m  \x1b[49m  \x1b[m",
"\x1b[49m    \x1b[48;2;0;0;1m  \x1b[48;2;135;135;135m  \x1b[48;2;255;255;255m  \x1b[48;2;255;255;254m  \x1b[48;2;254;255;255m  \x1b[48;2;250;250;250m  \x1b[48;2;249;249;249m  \x1b[48;2;142;142;142m  \x1b[48;2;152;153;152m  \x1b[48;2;255;255;255m        \x1b[48;2;254;255;255m  \x1b[48;2;255;254;255m  \x1b[48;2;249;249;249m  \x1b[48;2;253;253;253m  \x1b[48;2;255;255;255m  \x1b[48;2;229;229;229m  \x1b[48;2;69;68;68m  \x1b[48;2;249;249;249m  \x1b[48;2;205;205;205m  \x1b[48;2;231;231;231m  \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m  \x1b[48;2;243;242;242m  \x1b[48;2;122;123;122m  \x1b[48;2;0;0;0m  \x1b[48;2;1;0;0m  \x1b[49m    \x1b[m",
"\x1b[48;2;0;0;0m  \x1b[48;2;0;1;0m  \x1b[48;2;0;0;0m  \x1b[48;2;8;8;8m  \x1b[48;2;179;179;179m  \x1b[48;2;255;255;255m    \x1b[48;2;255;244;244m    \x1b[48;2;254;249;249m  \x1b[48;2;255;255;255m      \x1b[48;2;244;245;244m  \x1b[48;2;29;29;28m  \x1b[48;2;29;29;29m    \x1b[48;2;176;176;176m  \x1b[48;2;255;255;255m        \x1b[48;2;255;250;251m  \x1b[48;2;255;205;205m  \x1b[48;2;255;161;161m  \x1b[48;2;255;163;163m  \x1b[48;2;255;255;255m    \x1b[48;2;175;175;175m  \x1b[48;2;0;0;0m  \x1b[49m      \x1b[m",
"\x1b[48;2;1;0;0m  \x1b[48;2;0;1;0m  \x1b[49m  \x1b[48;2;43;43;43m  \x1b[48;2;181;181;181m  \x1b[48;2;253;253;252m  \x1b[48;2;254;255;255m  \x1b[48;2;255;181;181m    \x1b[48;2;254;213;213m  \x1b[48;2;255;255;255m    \x1b[48;2;254;255;255m  \x1b[48;2;255;255;255m      \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m          \x1b[48;2;255;227;227m  \x1b[48;2;255;184;184m  \x1b[48;2;255;181;181m  \x1b[48;2;255;225;225m  \x1b[48;2;255;255;255m  \x1b[48;2;240;241;241m  \x1b[48;2;143;143;143m  \x1b[48;2;0;0;0m  \x1b[49m      \x1b[m",
"\x1b[48;2;0;0;0m        \x1b[48;2;103;102;103m  \x1b[48;2;234;234;234m  \x1b[48;2;255;255;255m  \x1b[48;2;255;254;255m  \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m      \x1b[48;2;254;255;255m  \x1b[48;2;255;255;255m    \x1b[48;2;254;255;255m  \x1b[48;2;255;255;255m                  \x1b[48;2;255;254;255m  \x1b[48;2;254;254;254m  \x1b[48;2;154;155;155m  \x1b[48;2;0;0;0m  \x1b[49m        \x1b[m",
"\x1b[49m  \x1b[48;2;0;0;0m        \x1b[48;2;109;109;108m  \x1b[48;2;253;253;253m  \x1b[48;2;255;255;255m  \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m  \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m      \x1b[48;2;255;255;254m  \x1b[48;2;254;255;255m  \x1b[48;2;255;254;255m  \x1b[48;2;255;255;254m  \x1b[48;2;255;255;255m          \x1b[48;2;255;255;254m  \x1b[48;2;254;255;255m  \x1b[48;2;255;254;255m  \x1b[48;2;127;127;127m  \x1b[48;2;32;32;32m  \x1b[48;2;0;0;0m  \x1b[49m        \x1b[m",
"\x1b[49m  \x1b[48;2;0;0;0m  \x1b[49m    \x1b[48;2;0;1;0m  \x1b[48;2;0;0;0m  \x1b[48;2;86;86;86m  \x1b[48;2;118;118;118m  \x1b[48;2;255;255;255m                            \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m  \x1b[48;2;181;181;181m  \x1b[48;2;95;94;95m  \x1b[48;2;8;8;8m  \x1b[48;2;0;0;0m  \x1b[49m          \x1b[m",
"\x1b[49m            \x1b[48;2;1;0;0m  \x1b[48;2;31;31;31m  \x1b[48;2;52;52;52m  \x1b[48;2;58;58;58m  \x1b[48;2;155;155;155m  \x1b[48;2;245;245;245m  \x1b[48;2;255;255;255m        \x1b[48;2;255;254;255m  \x1b[48;2;255;255;255m  \x1b[48;2;254;255;255m  \x1b[48;2;255;255;255m  \x1b[48;2;254;254;255m  \x1b[48;2;207;207;207m  \x1b[48;2;80;80;80m  \x1b[48;2;52;52;52m  \x1b[48;2;47;47;47m  \x1b[48;2;13;13;13m  \x1b[48;2;0;0;0m  \x1b[49m            \x1b[m",
"\x1b[49m                \x1b[48;2;0;0;1m  \x1b[48;2;0;0;0m  \x1b[48;2;29;29;29m  \x1b[48;2;19;19;19m  \x1b[48;2;16;17;17m  \x1b[48;2;17;17;17m  \x1b[48;2;17;17;16m  \x1b[48;2;17;17;17m    \x1b[48;2;17;17;16m  \x1b[48;2;16;17;17m  \x1b[48;2;17;17;17m  \x1b[48;2;17;17;16m  \x1b[48;2;23;23;23m  \x1b[48;2;21;20;21m  \x1b[48;2;0;0;0m  \x1b[49m                  \x1b[m",
SPACE, SPACE, SPACE, SPACE, SPACE, SPACE
];

pub const ASCII: [Ascii; 1] = [Ascii {
    small: CAT_ASCII_MINI,
    big: CAT_ASCII,
}];