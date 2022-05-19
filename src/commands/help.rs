use crate::tools::gui::GUI;
use crate::traits::command::derive_getters;
use crate::traits::command::{Command, Runnable};
use anyhow::Result;
use std::collections::HashMap;
use std::str::FromStr;
use strum::{Display, EnumString};

pub struct Help {
    pub required_args: Vec<String>,
    pub params: HashMap<String, String>,
}

#[derive(Display, EnumString)]
enum Actions {
    #[strum(ascii_case_insensitive)]
    Help,
    #[strum(ascii_case_insensitive)]
    Default,
}

impl Command for Help {
    derive_getters!();

    fn print_title(&self) -> bool {
        false
    }

    fn help(&self) -> Result<()> {
        println!("
,,,,,;;;;;,,,,,;;::ccc::::cccc::ccc::;:clodxxxxxkkOOkkxxxkkkkkkkxxxkkO0000OOOO0KXXNNNNNNNNNNNNNNNNNNNNNNNNNNXXXXXNNNXXXXXNNNNXNNNNNNXXXXXXXXXXNNNNNXXXXXXXXXXNNNNXXXXXXXXXXXNNNXKK0KKXNNNNNWWWNXXXKKXXXK
'''',,,,,,,,,;;;;;;:::ccllllcc:cclc:;;;:loddxxxkkkOkkxxxxkkkkkkkkkkkO00KK00OOO0KXXXXXXXNNNNNNNXXXNNNNWWWWNNNXXXXXXNNNXXXXXXXXXXNNNNNXXXXXNNNNNNNNXXXXXNNNNNXXXXXNNXXXXXXXXXXXXXXXKKKXNNWWWWNNNNXXXXXXXKK
.''''''',,,,;;;;;;;;::clllllcccccc::;::clooodxxkkkkkxxxkkkkkkkkkkkkkO000000000KKKKKXXXNNWWWNNXK0KKNNWWWWWNNNXXXXXXXXXXXXXXXXXXNNXXXXXXXXXXNNNNNNXXXXXXXNNNNXXXXXNNNNXXXXXXXXXXXXXXXXNNWWWWWWNNXKKKXXXKKK
'''',,,,,,,,,,,,,;;:::cccccccccc::::clooooooodxkkkkkkkkkkOkkkkkkOOkkOO000000000KKKXXNNNNXX0OkxddxkO00KKKXXNNNNNXXXXXXXXXXXXXXXNNXXXXXXXXXXXXXXNNNNNNXXXXXXXNNNNNNNNNXXXXXNNNNXXKKXXNNWWWWWWWNNXKKKKKXXXX
''''''',,,;;;;;,,,;::::::cclllcccccccloodddxxkkkkkkkkkOOOOOOkkkkOOOOOOOO0000000KKXXNNXKOxoc:;,,;;::cllodkOKXNNNNXXXXXXXXXXXXXXXNNNNXXXXXXXXXXXNNNNNNXXXKXXXNNNNNNNNNXNNNNNWNNXXKKKXNNNNNXXNNNNNXXKKKXXXX
''...'',,;;;;;;,,,;;:::::cllllllllllcclodxkkOOOkkxxxkOOOOOkkkkOO00KKK00OOOO000KKXNNXKkdc,'...........',;:lxOKXNNNNNNXXXXXXXXXXXXXXXXXXXXNNNXXXXNNNNNXXXXXXXXXNNNNNNNNNNWWWNNXXKKKXNNNNNNNXXNNNNNXXXXXXXX
'''.'',;;;;;;,,,,,;;;;;;;:clooooooooooddxxkkkkkxxxxxkOOOOOkkkO0KKKKK00000KKKKKXNNNX0xl;..................,cdk0XNNWWNNXXXXXNXXXXKKKKXXXXNNNXXXXXNNNNNXXXXXXXXXXXXXNNNXXNNNNNXXKKXXNNWWWWNNNNNNNXXXXXXKKKK
'''.'',;;;,,,,,,,,,;;;,,;:clooooooddxxxdxxxkkkxdddxxkO00000OO00000OOO0KKXXXXXXNWNXOd:'...................';ldk0KXXNNNNNXXXXXXXXXXXXXXXXXXXXXXXXNNNNXXXXXXXXXXKKKXXXXXXXNNNNNXXKXXNNNNNNNNWWWNNXXXKKKKKKK
'''''',,,,,,,,,,,,,;;;;;;:ccloooddddxxxxxxkkkxddddxkO0KKK0000000OOkOO0KXNNXXXXXXKOd:'.....................';codxxkOO00KKKXXXXXXXXXXXXXXXXNNNXXXXNNNXXXXXNNXXXKKKKXXXXNNNNNNXXXXXXXXNNNNNNWWWNNNXXKKKKKKK
,,'',,,,,,,,,,,,,,,,;;:::::ccloddddddxxxkkkkkxxdxxkO0KKKK0000K000000KKXXNXX0Okdolc;,'......................'',,;;:cloodxO0KXXXXXXXXXXNNNNNNNXXXXXNNNXXXXXXXXXXXKKXXXNNNNXXXXXKKXXXXXNNNNNNNNNNNXXXKKKKKK
''',,,,,,,,,,,,;;,,'',;:::ccllooddddddxxxxxxxkOOOOOOO00KKKKKKKKKKKKKKXKK0kxo:;'.................'.................''',;:coxkO0KXXNNNNNNXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXNNNNNXXXKKXXNNNNNWNNNNNNNNNXXKK00KK
',,,,'''',,,,,;;;;;,,,;::ccccclodxxxxddddddxkO0000OOOO0KKXXKKKKKKKXXXK0ko:,....................''.....................'',,;:ldxOKXNNNNXXXXXXXXXXXXXXNNXXXXXXXXXXXXXXXXNNNNXXXXXNNNWWWWWWNNNXXXXXXKKK00KK
,,,,,''',,;,,,,;;:::;::ccc::::loxxkkxxddxxkkOOOOO000KKKKKKK00000KXXXKOdc'..  .......................................''''.....,;cdk0XNNNNXXXXXNNNXXXNNNNNXXNNNNNXXXXXXXXNNNXXXXNNWWWWWWWWWNNNXXXKK00KKKKK
;;;;;,,,;;;,,,,,;;:::clllcc::clodxxxxxxxxkO000OOOO00KKKKKKK00KKXXXK0xl;.. ........................................'',,''........,cdOKNNNNXXXNNXNNNNNNNNNNNNNNXXXXXNNNNNXXXXXXXNNNWWWNNNNNNNNNNXKK000000K
,;;;;;;;;,,,,,,,;;;:clooolcccllodxxkkkkkkOO00000OOOOO00KKXXKXXXXK0ko:'.............................................',,,,,'.......,:lxOKXNXXXXXXXXXXXXXNNNNNXXXXXXXXNNNXXXXXXXXXNNWWWNNNNNNNNNNXKK000000K
;;;;;;;,,,,,,,,;;;:ccllllccccloddxkkkkkOOOOO00000OOOO00KXXXXXXXKOxc,.................................''''...........';;:;,'........';okKNNXXKKXXXXNXXXNNNNNXXXXXXXXXXXXXXXXXXXXNNWWWWWNNXXXXNXXK0OOO0KKX
;;;;;,,,,,,,,,,,;;:cclcc::;:cldddxxxkkkkkkOOO0000000000KKXXXXXK0xl,...............................''''''............',;;,,'..........,lkKXNXXXXXXXXXXXXNXXXXXXXXXXXKXXXXNNNNXXNNNNNNWWNNXXXXXKK0OOOO0KXX
,,,,,,,,,,,,,,,,;:clooolc:;;:cooddxxkkkkkkkO000KKKKK000KKKXNNX0kl;......'''..............''......''.................',;;;,''..........':oOXNNXXXXXXXXXXXXXXXXXXXKKKXXXXXXXXNNNNNNNNNNNNNNXXXKK00OOOO00KK
,,,,,,,;;;::;;;;:cloddddollccllooddxkkkkkkO0KKKKKKKKKKKKKXNNNKko:'..'',;;;;,............',,;;,,;;;;,,''.'''''.......',,;;;,''...........;dKNNNXKKKKXXXXXKKKKKKKKKXXXXXXXXXXXNNNXXXXXXNNNNNXXKK0000000000
;;,,,,,;;:::::::cclooddooooooooodddxxxxxkkO0KKXXXNXXKKKKKXNWN0d:,',:ccllooolc;,''''''',,;:coodddddollc::;,,'...''''''',,;,,''......... .'o0XNNXXKKXXXXXXXXKKKKXXXXXXXXXXXXXXXXXXXXXNNXXXXXXKK00000K00000
;,,,,,;;;;;;:::::cllodddoooooodddxxxxxxxxkkOO0KXXNNXXXKKKXNNXOo;,,coxxxxxxxxddollllloooodxkO000Okxxdoolc:,'.........',;;:;,''........  .'lOXNNNNXXXXXXXXKKKKXXXNNNNXXXXXXXXXXXXXXNNNNNNXXXK0000KK00OOO00
;;,,;;::::;;;;::ccloddddooddddddxxxxxxxxxkkkOO0KXXXXXXXXXXNNN0d:;cokOOxdolloodxxxkkkkkkkOO000Okxdoolllcc:,'.........',;;,''.'........  .'ckKNNNNNNNXXKKKXXXXXXXXXXNNNXXXXXXXXXXNNNNNNNNNNXXK00KKKK00OO00
c:::::::cc::::ccllodddddddxxxxdddxxxxkkkkkkOO00KKXXXXXXXXNNNNKkoloxOOxoc:::cclooddxxxkkkkOOOOkxxdolcc::::;,'........',,'....''........ .'cxKNNNNNNXXXXXXXXXXXKKKKXXXNXXXXXXXXXXNNNNNNNNNNXXKK000KKKK0000
cccc:::::cccccclooddxxxddxxxxxdddddxxkkOOOOOO0KXXXXKKKKKXXNNNXOxdxxkxdlc:ccllooooodddxxkO000Okkxxxdlcc::cc:,'...''''''''''.'.......... ..:dOXNNNXXXXXXXXNNXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXKKKKKKKKKK00000
ccccc:::cccccccclodxxxxxxxxxxxxxxxxxxkkkOOOOO0KXXXXKKKKKXXNNNX0kxxxkkxddoodddxxxxxxxxkO0KKK0Okxxxxxdoolllc:;,'''''''''''...''............,lxOKXNNNXXXXXNNNXXXXNNNNXXXXXXXXXNXXKKKXXXXXKKKKKKKKKKK00OOO00
llccc:::cccccccclodxxkkkkxxxxkkkkkkkkkOOOOOkOO0KXXXXKKKKXXNNXKOdolooddxxxxxxxkkkkkkxxxkOOOkkxdollllllllllcc:;;,,''''''......'',''........,:lxOKXNNNXXXXNNXXXXXNNNNXXXXXNNNNNXXXXXXNNNXXXXXXKKKKKK00OOOOO
oollc:;;:cclcccclodxkkOOOOOkkkkkkkkOOOOOOOOkkkO00KKXXXKKXXNNX0xc;,,,;:cldxxxdddxxxdolllllcccccc:;,,,,;::cccc::;,''..........,;;;,'........,:ok0XNNNNXXXXXXKKKXNNNNXXXNNNNNXXXXXXXNNNNXXXXXXKKKKKKK00OOOO
ooolc:;;:cccccccloxkOO000OOOOOOOOOkkkOOOOOOOOOO00KKKKKKKXXNNX0xl;,'''',,:cllcccccc:,,''''',,;;:;,''..'',;:ccc:;,'..........;cll:,''''... ..;lx0XNNNNNXXXXXXXXXXNNXXXXNNNNNXXXXXXXXNNXXXXXXKK000KKKKK00OO
llllc::::cccc::ccodxOO00000OO0000OOkkkkkOOOO000KKKKKKKKXXXNNNX0koc:;,,''',,;;;,,''.......',;::::;,''..'';:cccc:;,,'.......';cll:;,''''.....,cx0XNNWNNNNXXXXKXXXXXXXXXXNNNNXXXXXXXXXXXXXXKKKKKK00KKKKK000
ccccccccccccccccllodxkO0KKKKKK00OOOOOOOOOOOOO00KKKKKKXXXXNNNNNXKkl;,,;::;,,;::;,....'''.'';:ccc:;,'....';:cccccc:;,'.. ......'',;:;,'......,:okKXNNNNNNNXXKKKXXXNNNXXXXKXXXXXXXXXXXXXXXKKKKXKKK0KKKKK00O
ooooolllcccccllllloodxkO0KXXKK0000000000OOkkkOO00KKKKXXXXNNNWWX0xc,',:lol::cool;'..'',''...';:cc:,'.....,;:ccccc:;,'.   .......',:cc;'.. ..';ok0XNNNXXXXXXXXXXXNNNNXXXXKKKXXXXXXXXXXXXXXXXXXXXKKKKKKK00O
dddoooollcccccclllloodxk0KKXXKKK0000000000OkkkkkOO00KXXXXXNNNNX0xc,',:cllcclddo:,'',;::;,'..,:::;,'....',;::ccc::;,..   ........';:cc;..  ..;oOKNNNNXXKXXXXNNNNXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXKKK0KKKK00O
dolcccllolllccccllloodxkOO0KKKKKKKK00KKKK0OkkxxxxkO0KXXXXXXNNNX0koc:;;::::clool:;;:clodolc;;:ccc:;;;;;::cc:::::::;,'.    ........,,;,'..  .'cx0XNNNNNXXXXXXNNXXXXXXXXXXXXXXXXXXKXXNNNXXXXXXXXKK000KKK0OO
xdlc:cclllcccllllloddxxkkOO00KKKKKKK0KKKK00OkxxxxxkOKXXNXXNNWNX0kdolccccclllllc:;;:cldxkkxdooodddddxxddolc::;;;;;;,'...  ....',,,,'........,lkKNNNNNNNNNNNXXXXXXXXXXXXXKKKKKXKKKKXNNNNXXXXXXXXKKKKKKK00O
dddollcccccllllllloddxkOOOOOO0KKXXKK000K000OOkxdddxk0KXNNNNWNXOxolllllodxkkdl:;,,;;:codxkOOOkkkO0000OOxoc:;,,,,'''.........';ccc:,'..  ...':d0XNWNNNNNNNXXXXXXXXXNNNXXXKKKKKKKKKKXNNNXXKXXXXXXKKKKKKKK00
ldxxxdlcccclooollooddkO00OOO00KXXXXKK0000000OkxddddxO0KXXXNNX0dc;;:loodk0K0xl;,,;:cllllodk0KKKK0OOOkkxoc;,'................,:ccc;,..   ..,cx0XNNWNNNNNNNNXXXXXXXXXXXXXXXXKXXXKKKKXXXXXKKXXXXXXKKKKKKKKKK
lodxxdl:::cloddddooddxkOOOOO0KKXXXXXKKKKKKK00OxddddxkO000KXNXOl,.';ldddxkOOxl;,;:lodolc:coxO000OOkxddoc;'...................''''....   .,lk0XNNNNNNNXXXNNNXXXXXXXXKXXXNNXXXXXKKKKKKKXXXXXXXXXXXXKKKKKKKK
llooollcccclloooooodxxkkkkkO0KKXXXXKXXXKKKK0OkdoodxkOOOO00XNXkc'..,coolllolc:;;;:clllcc::::clddxddolcc;'..............    ..........  ..:xKNNNNNXXXXKKXXNNNXXXXXXKKXXXXXXXKKKXXXXXXKKKXXXXXXXXNNXXK0000K
clllllllllllcclloooddxkkkkO00KKKXXXXKKKKKKK0OkdoodxkO000KKXNKxc...';ccc:,,'''',,,,,,,:cccc:::::cccc::;,'...................   ....... .':xKNWNXXXXXXXXXXXNNXXXXXXXXXXXXXXKKKXXNNNNXXKKKKKKXXXXNNNXK00000
llllllllllllclloddddddxkkO0KKKKXXXXXKK0000KK0OxddxkkOO0KKXNNKx:...';cc:,'.....'''....';cloolc:;,;;;;;;,'................... .........  .;dKNNNXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXKKKKKKKKKKKXXNXXXKKKK
oooooooollllllooddddddxkOO00KKXXXXXXKK000KKK0OkxxxkOO0KKXNNN0d:...';;;'................,;::;;,,',,,;;;,'.................. ..........  .,oOXNNNXXXXXKKKKKXXXXXXKKKKKXXXXXXXXXXKKKKKKKKXXXXKKKKKXXNNNXXKK
lloooooollllllodddooodxOOOOO00KKKKKKKKKKKK0OkkkkkkO00KKXXNNX0d:..................................'''''.................... .............,oOKNNNNNNXXKKKKKXXXXXKKKKKXXXXXXXXXKKKKKKKKKXXXXXKK000KKXNNXXKK
ccllloollcclloddddollok0KK0OOO0KKKKKKKKKK00OkkkkO000KKKXXXXKOo;..  ......''',,,,'''.....................................................;o0XNNXXXXXXXXXXXXXXXXXXXXXXNNXXXXXXKKKK00KKKXXXXKK00OO0KKXXXXKK
ccccclllllcllodddolccok0XXK0000KKKXKKKKK00OOOOOO000KKKKXXXX0xl;..  .....',;:clllcc::;,'........................................',,'....,:d0XXXXXXXXXNNNNNXXXXXXXXXXXXXXXXXKKK00000KKXXXXKK0OOkkO0KKXXXXK
cccccclllllllloooolclok0XXXKKKKKXXXKKK000OOOOO0000KKKKKXXNXKkl;.   .....',,;:clcccc:;,'........................................',,,''',:lxOKKXXXXXXXXXXXXNXXXXXXXXXXNXXXXK00OOOOO0KXXXXXKK0OOkkkO0KXXXXX
lcccccccccclllloollcldk0KXXKKKKXXXXKK00OOOOOO00000KKKKXXNNNX0d;.   .......'',,,''',,,'........................................',,;;;;:cldk0KKXXNNNXXKKKXXXXXXXXXXXXXXXXXXK00OOOOO0KXXXXXXKK0OkkkO0KXNNXX
cccc::::::cloooolccccok0XXXXXKKKKKKKK000000000000000KKXXNWWNKxc.. ........'',,'...........'''.................................',::cccloxk0KXNNNNNXXXKKKKXXKKXXXXXXXXKKXXKKKK0000KXXXXXXXXXKK0OOkO0KXNNNN
cccccc::;:clooollc::cok0XXXXXKKKKXKKK000KKXXXK0OOOO00KXXNNNNKkc'.....''''',,;;;,'.......'',''..................................;:lllodkOKXXNNNNNNXXKKKKXXXXXXXXXXXKKKKKKKKK0000KKXXXXXXXXXXXK0OOOO0KXNNX
:cllllc::cccccccccccldk0XXXXXXKXXXXKKK0KKKXXKK000000KKKXXXNXKkl,....''''',,;;;;,'.....'','''..'''.............. ..............,:clooodxO0XXNNNNNNXXXKKXXXXXXXXXXXXKKKXXXKK0OOOO0KKKKKKXXXXXXK0OOOO0KXXXX
::cccc::cccc:;;::codxO0KXXXKKKXXXXXKK000000000000000KKKKKXXXKkl,........'',;;;,'.....',,,'..'',,''.........................',;:clllllloxk0KXXXXXXXXXXKKKKXXXXXXXXXXKKXXXXK00OO00KKKKKKKKKKKKK0OOO00KXXXX
::::;;;;:clcc::cloxO0KKKXKKKKKXXXXXKK000OOOO0000000OO000KKXXKkl,........'',,,,,'......'''....',,,'........................',;::cccccclodxk00KKKXXXXXXKKKKXXNNXXXXXXXKKKKKKKK00000KKKKKKKKKKKK0OOOO0KKXXX
:::;;;;:clloooodxkO0KXXXKKKKKKXXXXKKKK00000000KKK0000000KXXXKkl,........'''',,'''......''...''''''........................'',,;:ccccclloodxxk0KXXXXKXXXXXXXXXXXXXXXKKKKKKXXXXK00000KKXXXXXXXK00OkOO0KKXX
:::::ccllooddxxkkO0KXXXXKKKKKKKXXXKKKKKK000000KKKKKKKKKKKXXX0xc'.........''''''''......',,''''''..........................'',,;:cccccccccclodxO0KKKXXXXXXXKKKXXXXXKKKKKKKXNNNXK0000KKKKXXXXXXK0OOOO0KKXX
:::cclodddxxxxkkOO0KXXXXXKKKKKKKXXXKKK000000000KKKKKKKKKKXXX0xc,.........''''''''......'''''''''''''''...................'',,;::cclcc:;;;:cllodxkO0KKXXXKKKKXXXXXXXXXXXXXXXXXXKK00KKKKKKXXXXKKK0000KKKXX
:::clodxxxxxxxxkkO0KKXXXXXXXKKXXXXXXKKK000000OOOOO000KKKXXXKOxl;'.......'''''.'''.....''''.....'''''''.............''....'',,;::clllc:,,,;:lloooodxxkkOO0KKXXNNNNNNNNNNNNNNNXXKKKKKKKKKKKXXKKKKKKKKKKKKK
::ccldxxxxxxxkkO000KKKKXXXXNNNXXXXXXKKKKKK000OOOO000KKXXKK00kxoc;'..............'.....''''..........................''''''',;;::clllc:,',;:cloooooooooodxkkO0KKKXXXXXXXXNWWWNNXXKKKKKKKKXXXKKKKKKKK0KKKK
ccclodxxxxdxxkO0KKKKXXXXXXNXXXXXXXXXXXXXXXXXKKK00KKKXXXK0kxxddlc;,.....................''.........................'''',,,,,;::::::clcc:;;;::cllodddolllcccccllodxxkkkO00KXNNNNNNXXKK0000KKXXXXXXKK00KKXX
llcccldxxkkkkOOO0KKXXXXNNXXXKKKKKXXNXXXXXXXXXKK000OOOkkxolccclc:;,'..............................................',,;;:;;;;::cc::::ccc::;::cclloooollcc:::;;;;::clloooddxxxkO00KKXXXK0000KXXNNXXKKKKKXXX
ollcccloxkOOOOOkOO00KKKXXXXXKKKKXXXXKK00OOOOOkxxdddoolc::;;;;::;;,'...........'.................................'',;:cc::;;;;:::::::::::::ccccc::;;::::::cccc::::cllolllllllllodxk0KXXXXXXXXXXXXXXKKKKK0
oooollloddxkOOOOkkkOOO00KKXNNNNXXK0Okxdoooooolccccclcc:;;;;;;;;,,,''...........................................'',,;::::;;;,,,,,;;::cclllccc::;;,,,;;;::clllc:;;::lloodxxkxdlc;;;:ldk0KXXXXXXXXXXXXKK000
oooooooooooxkOOOkkkkOkkOO0XNNNNK0kdolc:::::::::cclllc::;;;:ccc:;,,''''''',''.........'''......................''',,,;;;;;,,,,,,,;;;:ccloooolc::;;;;;;;:ccccc:;;,,;;:codk00Okdlc:;;;:ldxOKXXXXXXXXXXKKKKK
lllllooollodxkOOkkkkkkkkO0KKKK0Oxdolc:;;;,,,,;:loddoc:,,,;:clllc:;,''',,;;;,'.......''''..................'''''''''',;;;;;,,;;:::cccccllodddoolllcccccccllcc::;;,,,,;:loxxxddollc::;::ldkO0KXXNNNNNXXXKK
lllllloolllodxkkkkkkkkkkO000Okxdooollc:;;,''',;cloolc;,,,;;:clllc:;,,;;;;,''........'''.................'''''''''''',;:::::;::clloooolllloddddddoooooolllllccc:::;;,'',,;:cccclllcc::;::loxkOKXXNNNNNXXK
cclllllllllloddxkkkkkkxxkkOOkxdolc::;;::::;,,,;:cc::;;;;;;;:ccllc:;;;;;;,''''''...........''''''''''''''',,,,,,,,,,,;;;::::::cclooddddoolllooddooolllllccccllllllc:;,'''',,;::cloolc::;;;:cldxkO0KKXXXKK
llllllllolllllodxkkOOkkxddddxxdoc;,'',;cccc;;;:::::;;;;;;:cccccc::;;;;;;;;,,,,'.......'''',,''''''',,,,,,,,,;;;,,,;;;::::cccccllooooooolllllooollcccccc::cclloooolc:;;,,,,,;:clloollc::;;;:cloodxxkO0KKK
llllllcccccccclodxxxkkkxdolclllc:;,,,;:cllc:;;;:clllc::::ccllcc::::::::::::;,'.....'',,,,,,,,;;,,,,,,,;;;;;;,,,,,;;::ccccccccllllcc:::cccccllllcccccccc::clllllllllccc::::::ccccccccc:::::::cloodddxkO00
;;:::;,''',;cldddddooddxdolc:::;,''',;:ccc::;;:clodolcc:::ccclllcc:::::::c::,''''',,,,,,,'',;:::;,,,,,;;::;;;;;;;:::::::::::::cc:::;;;::::::::::::::cc::::cccc::clllllccclllllcccccc::::;;;;;:lddxxddddd
',,;;,'....,:lodddooddxxdolcc::;,'..',;:c::;,,;:clllllcc:::::cllllccc:::cccc:;;,,,,,,,,,',,,;;;;;;,,,,,;,,;;;::::::::::::::::::::::::::::;;;;;:;;,,;;;;::::::::cclllcccllooollc::::::::::;;,,;coxxdolc::
,;::::;,''',;:cloodxkkkxdlcc:::;,'',;::cc:;,,',,;;::ccccc::::cclloolllcccclllcc:;,,,,,,,,;;;;;;;;;;;;;,,,,;;:::::::::cc:::::ccccc::cc::;;;;;;:::;,,',,;:::::::ccccccccccllllcc::::::::::cc:;;:codol:;;,,
::cccc:;,,,;:cloodxkOOkdoc;,,,;,,,;clllc:;,,''',,;;;;;;:coooollllllooollllllllc:;,'''',,;;;;;;;::c::;;;;;:::::cccc:cccc:;;;;:cc::;;::;;;;;;::::::;;,',;:cccccccccc::c:::;;;::::::ccc:::::c::::clll:;,,,,
:ccccc::;,,;codxxxxxkkkxoc;''',,;;:loooc:,''''',,;;;,,;:coddollllllooollllloolc:;,,,,,,;;::::::::::;;;::cccccccc::::ccc:;;;;:cc:;;;;;;;;::ccc:::cc:;,,,;:ccccllc::::::;,,,,;::cc::::;;;;;:;;;;:cllc::;;;
cclcccc:;;;:loxkkxxxxxxxoc;,'',,;;:clllc:;'...',,;;;;;;:clcc::::cccclllcclloddoc:;;;;;;;;:cccc:;;,,;;;:ccccccc:::::::c:::;;:ccc::;;;;::cclllcccccc:;,,,;::::clllc::ccc:;,',;::::::::;;;;;;;,,,;:lolllcc:
looolc::;:clodxkkOkkxxdol:;,',,;;::::cccc:;'...',,;;;;::cc:,'',;;:;;:cccccclodol:;;;;,,,;;:::::;;,;;;::ccc::::::::::::::;;;::c::;;;;::ccllllcccccc:;;;;:::::::cccclloolc;,;;:::::clcc:;::::;,,,;cloooolc
llool:;;:coddxxkkkOkxxdolc:;,,,;:::;;;;:ccc:;,''',,,,;;:c::,'',;;;;;;:cllcccccc:;;,,,,,,;;;;;;:::::::ccccc::::::::::::::::ccllcc:;;;;:::::cccllcccccccccccc:::::::cloollc::cc::::clollclllllc:;::clooooo
llllc:::codxkkxxxxkkkxxdoc:;;;;:ccc::;;::cccc:;;,,'',,,;;:::;;;;;;;;:ccllcc:::;;;;;;;;;;;;;:::ccccc:::::ccc:;;;;;;;;;;;;::ccllllcc:::;;;;;;::cclllllllllllccc::;::cclllllllccccclodddooddddolcc:::cclodd
dolccclooodxkkkkxddxkkdolc:::::ccloollcc::::cclc:;,,'',;;::cc:;;,;;::::ccccc::;;;:::;;;;::cccccccc:;;;;;:cc:::;;;;;;;;;;;,;;;:cclllcc:;;;;;;;:cloodoolooollcc:::ccccllllolllllodxxxxxxdddooolccccccclood
dolcccoodddxkOOkxddxkxdolllllc:::clooolcc::cclllcc:;;;::cccc::;;;;;;;;:::::::::;;;;;;;;;::cc:::::::;,,,,;::ccccc:;;;::c:;,,,;ccccccllc:;;;;:::ccloddddoooollc::::ccccclloooollooddxxxxddooooolllcccllood
xdolllloodxkOOOOkkxxxdolloooolc::cllllllcllllc:::::::::cclc::;;;;;;;;::::;;::::;,,,;::;;;;:;;;;::cc:;;,,,;;:ccc::;;:cllc:,,,:cc:::coolc;,;::ccccclodxxddolllcc:::cccllloddoolcllodxxxxxxdddddoooollloodd
xxxddoooddxkOO00OOkkxddooodddooolllllllllllllc:::::;;;;:clc:;;;;;;;;:cccc:;::c::;;;:c:::;:::;;;::cccc:;,,;;:ccc:;;;:cllc:;,,,;;;;:clol:;,,;:cccccclodxxdolcc::::ccllooddxdollclodxkOOOOOkkxxddddxxdddddd
odxxxxxxxxxkOO00OOkxxxxxxxxxxxxdolllooddooollllllc:;,,;:cllc:;;;;,,;;:ccc::::cc::::::;;;;:::;;;;::ccc::;;;:cclcc:;;;;:ccc:;;;;;;:::::ccc::;;::cclllooodddolc::::ccclloddxxxddodddxxkOO0000Okxddxxxxxxkkk
clodxxxxxxkkkkOOOOkkxkkkkkkkkkkkxxdxxxxxxdolllloolc:;;,;:ccccc:;,,,,,;:::;;;::ccc::;,,,;;:::;;;;;::::::;;;:cllollc:;;::ccc::;;::::;;;:cllllcccclooollldxkkxol::;::ccclodxkkkxxxddddxxkOO000OkkxxxkkkOOOO
::cclodxxkkxxxkkOOOOkkkkkkkkkkkOOOOOOOkkxxolccclooddol::;:ccllc:;;,;;::::;;;::cc:;,''',;::::;;,,,,,;;;;:::ccllooollccllllc:;;;;:::;;;;;cloddddddddoooodxkkkxoc;;,;::clodxkkkkxxxxxxdddxxxkkOOOOOkkkkkkxx
;:::ccllodxxxxxkOOOOOOOOOkkxxkkkkkkOOOOkkxdollllodkkkxdooloooolcccccclccccccclllc;,'',;:ccc:;;,'''''',;:clllcccccclloddddolc:;;::c:::;;:coxkOOkkxxdddxxkkOOkdl:;,;;:clodxkOOkkkkkxxxxddddxxxkkkxxxxddool
:::::;;;:clodddxxkOO000OOkxxxxkkkkkOOOOOkxddoooddxkkkkxxxxxxddoooooooooooooodddol:;;;:cccc::;::;,,,,,,;;:ccccc::;;:cldxkkxolcccclllllcccloxkkOOOkkkOOOOOOO0OkdlccloodxxkkOOkkkxxxxxxkkxxxdddddooooooooll
::;;;;;;;;;::ccllodkO000OOkxxxxxxxkOOOOkkkxxxxxxxxxxxxxxkkkkxdddddddddxxxxxxkkkxdolcllllllc:cccc::ccclccc:cllllc:::cldxxxdooooooddddddooodddxxxkkkkkOOOOOOOOkkkkkkOOO00OOOkkxxxxxxxxxxxddooooollllllllll
c:::::cc:;,,,;;;:ccodxkkOOOkxxddddxxkkkkkkkOOOkkkkkkkkkkOOkxdooooodddxxxxkkOO0OOkkxxxdddddooolc:::loddxddodddddoooodddddddooooooddddddddddooddddddooddxxkkkkkkOOOOOOOOOkkxxxxxdddoollllcccclooolllcllloo
lllcccccc::::;;;;:::cclodxxxddddddxxxxxddxkOOOkkxxxkxxxxxxdoollllloodddddxxkkkkkkkkkkkkxxxxxddoolllooddxxxxxxxddddddddoooooolllllllllllllllllooooollooooooddddddxxxxxdddoooollllllllcccccllooddooolloooo
ccccccllllolllccccc:::ccllllloodddddddooooddxxxxxddddoodooolllllllllllllooooooodddddddddddddxxkkxxxddddooodddddolllcclllllllllllcccccccccllllllllllllllccclllcccllllllllccccccccllloolloodddddooolllllcc
::;;;:cllooollllllllccccccccllooooollllllcllloddddooooooooollllccccccccclllcccccllllccllllloodxxxxxxxxddoooooooollccccccccccllllllllllllllllllllccc::ccccccccccccccclllllllooolllloooolloooolcc::;;,,,,,
c:;''',;;;;;;;;;;:::cccccccllooooollllllllcclllooodooooooooollccccccccccccccccccclllllcccccclloooddddddddoooooooooooollllcllllllllllllllllllllllllccclllllllllllllllooooolloollcccccc::;;;;,,,,'''''....
:;,..............'',,,;;;:::::ccllllccccllllllooooooooooddddollllllllcccllloolllllloolllccclloooooooooodddddddooooodddddooooooooooolllllloolloooooolllllcccccccccccclllcc::::;;,,,,,'''........'''''''''
........................''''''',,;;;,,,;;::cccllcc:::clloooollccccccccccclllollllccclcccccccllllllllllllloooolcccclooolllllllllllllllllllllllllllllc::::;;,,,,,,,,',,,,,,,''''...'''..........',,,,,,,,,
................................''''...'',,,,,,,,,''',,,;;;,,,,,,,,,,,,,,,;;;;;;;;;;;;:::;;;,,,;;:::::;;;;;;;,,,,;:::;,,,,,;;;;,,,,,,,,,,,,,,,,;;;;;,,,,,''..'''.......''''''....''''''',,,''',,;;;;;;;;
''''''''....................''''........''..................''.............'''''''''',,,,,,'''',,,,,,,,,,,''''',,,;;;,''....'''................''''...'''''..'''''..''''''',,,'',,;;;;,;;;;;;;;;;;;;;;;,
',,,,,,,'..........'',,''...''''......'''...................''''''''''''''''''''''''',,,,,,;;;;;;,,,,,,;;;;,,;;;;;;;;;;,,'''''''''',,'''''','''........'''...''',,,,,,,,,,,;;;,;;::cc::;;;;::;;,''''',,,
''''''''...'''''',;;:ccc:;,'',,''...'''''''''''''''............'''''',,,,,,,,,'''',,,,;;;;::::;;;,,,,,,;;;;;;;;;;;;;;;;;;,,,,,''''',,,'''',,''....''.''''....',,;;;::::::::;;;;;:ccllcc:;,,,,,''.....',,
");
        Ok(())
    }
}

impl Runnable for Help {
    fn run(&mut self, action: &String, _params: &[String]) -> Result<()> {
        let action = Actions::from_str(action).unwrap_or(Actions::Default);

        match action {
            Actions::Help => self.help(),
            Actions::Default => self.show_help(),
        }
    }
}

impl Help {
    pub fn show_help(&self) -> Result<()> {
        GUI::new()
            .title("Welcome to Rusty Search")
            .sub_title("commands:")
            .nl()
            .content("manage:   actions to create and delete indexes")
            .content("edit:     actions update and manipulate index data")
            .content("search:   actions to search index data")
            .content("rank:     actions to set search ranking settings")
            .content("config:   actions to change config settings")
            .nl()
            .sub_title("Run `{command} help` for specific help")
            .nl();

        Ok(())
    }
}

impl Help {
    pub fn new() -> Self {
        Self {
            required_args: Vec::new(),
            params: HashMap::default(),
        }
    }
}
