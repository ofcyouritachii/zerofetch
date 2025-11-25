use colored::*;

pub fn get_logo(distro: &str) -> Vec<String> {
    let distro_lower = distro.to_lowercase();

    if distro_lower.contains("ubuntu") {
        ubuntu_logo()
    } else if distro_lower.contains("debian") {
        debian_logo()
    } else if distro_lower.contains("arch") {
        arch_logo()
    } else if distro_lower.contains("fedora") {
        fedora_logo()
    } else if distro_lower.contains("macos") || distro_lower.contains("darwin") {
        macos_logo()
    } else if distro_lower.contains("windows") {
        windows_logo()
    } else if distro_lower.contains("freebsd") {
        freebsd_logo()
    } else if distro_lower.contains("openbsd") {
        openbsd_logo()
    } else if distro_lower.contains("android") {
        android_logo()
    } else if distro_lower.contains("gentoo") {
        gentoo_logo()
    } else if distro_lower.contains("manjaro") {
        manjaro_logo()
    } else if distro_lower.contains("mint") {
        mint_logo()
    } else if distro_lower.contains("centos") {
        centos_logo()
    } else if distro_lower.contains("alpine") {
        alpine_logo()
    } else if distro_lower.contains("nixos") {
        nixos_logo()
    } else if distro_lower.contains("opensuse") {
        opensuse_logo()
    } else {
        linux_logo()
    }
}

fn ubuntu_logo() -> Vec<String> {
    vec![
        "             ${c1}_".red().to_string(),
        "         ${c1}---(_)".red().to_string(),
        "     ${c1}_/  ---  \\".red().to_string(),
        "    ${c1}(_) |   |".red().to_string(),
        "      ${c1}\\  --- _/".red().to_string(),
        "         ${c1}---(_)".red().to_string(),
    ]
}

fn debian_logo() -> Vec<String> {
    vec![
        "       ${c1}_,met$$$$$gg.".red().to_string(),
        "    ${c1},g$$$$$$$$$$$$$$$P.".red().to_string(),
        "  ${c1},g$$P\"\"       \"\"\"Y$$.\"".red().to_string(),
        " ${c1},$$P'              `$$$.".red().to_string(),
        "${c1}',$$P       ,ggs.     `$$b:".red().to_string(),
        "${c1}`d$$'     ,$P\"'   ${c2}.${c1}    $$$".red().to_string(),
        " ${c1}$$P      d$'     ${c2},${c1}    $$P".red().to_string(),
        " ${c1}$$:      $$.   ${c2}-${c1}    ,d$$'".red().to_string(),
        " ${c1}$$\\;      Y$b._   _,d$P'".red().to_string(),
        " ${c1}Y$$.    ${c2}`.${c1}`\"Y$$$$P\"'".red().to_string(),
        " ${c1}`$$b      \"-.__".red().to_string(),
        "  ${c1}`Y$$".red().to_string(),
        "   ${c1}`Y$$.".red().to_string(),
        "     ${c1}`$$b.".red().to_string(),
        "       ${c1}`Y$$b.".red().to_string(),
        "          ${c1}`\"Y$b._".red().to_string(),
        "              ${c1}`\"\"\"\"".red().to_string(),
    ]
}

fn arch_logo() -> Vec<String> {
    vec![
        "                   ${c1}-`".cyan().to_string(),
        "                  ${c1}.o+`".cyan().to_string(),
        "                 ${c1}`ooo/".cyan().to_string(),
        "                ${c1}`+oooo:".cyan().to_string(),
        "               ${c1}`+oooooo:".cyan().to_string(),
        "               ${c1}-+oooooo+:".cyan().to_string(),
        "             ${c1}`/:-:++oooo+:".cyan().to_string(),
        "            ${c1}`/++++/+++++++:".cyan().to_string(),
        "           ${c1}`/++++++++++++++:".cyan().to_string(),
        "          ${c1}`/+++o${c2}oooooooo${c1}oooo/`".cyan().to_string(),
        "         ${c2}./${c1}ooosssso++osssssso${c2}+`".cyan().to_string(),
        "        ${c2}.oossssso-````/ossssss+`".cyan().to_string(),
        "       ${c2}-osssssso.      :ssssssso.".cyan().to_string(),
        "      ${c2}:osssssss/        osssso+++.".cyan().to_string(),
        "     ${c2}/ossssssss/        +ssssooo/-".cyan().to_string(),
        "   ${c2}`/ossssso+/:-        -:/+osssso+-".cyan().to_string(),
        "  ${c2}`+sso+:-`                 `.-/+oso:".cyan().to_string(),
        " ${c2}`++:.                           `-/+/".cyan().to_string(),
        " ${c2}.`                                 `/".cyan().to_string(),
    ]
}

fn fedora_logo() -> Vec<String> {
    vec![
        "             ${c1},'''''.".blue().to_string(),
        "            ${c1}|   ,.  |".blue().to_string(),
        "            ${c1}|  |  '_'".blue().to_string(),
        "   ,...._    ${c1}|  | |".blue().to_string(),
        "  ${c1}.  ${c2}.'${c1}.  '.${c1}|  | ${c2}|".blue().to_string(),
        " ${c1}|  ${c2}|-${c1}|   | ${c1} |  | ${c2}|".blue().to_string(),
        " ${c1}|  ${c2}'-'   /  ${c1}|  ${c2}|${c1}  |".blue().to_string(),
        "  ${c1}\\     ${c2}'    ${c1}|  | ${c2}|".blue().to_string(),
        "   ${c1}\\${c2}'-___-'${c1}  |  ${c2}|${c1} |".blue().to_string(),
        "    ${c1}'.    ${c2}.  ${c1}|  | ${c2}|".blue().to_string(),
        "      ${c1}'-.,___,'  | ${c2}|".blue().to_string(),
        "                ${c1}|_${c2}|".blue().to_string(),
    ]
}

fn macos_logo() -> Vec<String> {
    vec![
        "                    ${c1}c.'".bright_white().to_string(),
        "                 ${c1},xNMM.".bright_white().to_string(),
        "               ${c1}.OMMMMo".bright_white().to_string(),
        "               ${c1}lMMM\"".bright_white().to_string(),
        "     ${c1}.;loddo:. .olloddol;.".bright_white().to_string(),
        "   ${c1}cKMMMMMMMMMMNWMMMMMMMMMM0:".bright_white().to_string(),
        " ${c2}.KMMMMMMMMMMMMMMMMMMMMMMMWd.".bright_white().to_string(),
        " ${c2}XMMMMMMMMMMMMMMMMMMMMMMMX.".bright_white().to_string(),
        "${c3};MMMMMMMMMMMMMMMMMMMMMMMM:".bright_white().to_string(),
        "${c3}:MMMMMMMMMMMMMMMMMMMMMMMM:".bright_white().to_string(),
        "${c4}.MMMMMMMMMMMMMMMMMMMMMMMMX.".bright_white().to_string(),
        "${c4} kMMMMMMMMMMMMMMMMMMMMMMMMWd.".bright_white().to_string(),
        " ${c5}'XMMMMMMMMMMMMMMMMMMMMMMMMMMk".bright_white().to_string(),
        "  ${c5}'XMMMMMMMMMMMMMMMMMMMMMMMMK.".bright_white().to_string(),
        "    ${c6}kMMMMMMMMMMMMMMMMMMMMMMd".bright_white().to_string(),
        "     ${c6};KMMMMMMMWXXWMMMMMMMk.".bright_white().to_string(),
        "       ${c7}.cooc,.    .,coo:.".bright_white().to_string(),
    ]
}

fn windows_logo() -> Vec<String> {
    vec![
        "${c1}        ,.=:!!t3Z3z.,".blue().to_string(),
        "${c1}       :tt:::tt333EE3".blue().to_string(),
        "${c1}       Et:::ztt33EEEL${c2} @Ee.,      ..,".blue().to_string(),
        "${c1}      ;tt:::tt333EE7${c2} ;EEEEEEttttt33#".blue().to_string(),
        "${c1}     :Et:::zt333EEQ.${c2} $EEEEEttttt33QL".blue().to_string(),
        "${c1}     it::::tt333EEF${c2} @EEEEEEttttt33F".blue().to_string(),
        "${c1}    ;3=*^```\"*4EEV${c2} :EEEEEEttttt33@.".blue().to_string(),
        "${c4}    ,.=::::!t=., ${c1}`${c2} @EEEEEEtttz33QF".blue().to_string(),
        "${c4}   ;::::::::zt33)${c2}   \"4EEEtttji3P*".blue().to_string(),
        "${c4}  :t::::::::tt33.${c3}:Z3z..${c2}  ``${c3} ,..g.".blue().to_string(),
        "${c4}  i::::::::zt33F${c3} AEEEtttt::::ztF".blue().to_string(),
        "${c4} ;:::::::::t33V${c3} ;EEEttttt::::t3".blue().to_string(),
        "${c4} E::::::::zt33L${c3} @EEEtttt::::z3F".blue().to_string(),
        "${c4}{3=*^```\"*4E3)${c3} ;EEEtttt:::::tZ`".blue().to_string(),
        "${c4}             `${c3} :EEEEtttt::::z7".blue().to_string(),
        "                 ${c3}\"VEzjt:;;z>*`".blue().to_string(),
    ]
}

fn linux_logo() -> Vec<String> {
    vec![
        "        ${c1}#####".white().to_string(),
        "       ${c1}#######".white().to_string(),
        "       ${c1}##${c2}O${c1}#${c2}O${c1}##".white().to_string(),
        "       ${c1}#${c3}#####${c1}#".white().to_string(),
        "     ${c1}##${c2}##${c3}###${c2}##${c1}##".white().to_string(),
        "    ${c1}#${c2}##########${c1}##".white().to_string(),
        "   ${c1}#${c2}############${c1}##".white().to_string(),
        "   ${c1}#${c2}############${c1}###".white().to_string(),
        "  ${c3}##${c1}#${c2}###########${c1}##${c3}#".white().to_string(),
        "${c3}######${c1}#${c2}#######${c1}#${c3}######".white().to_string(),
        "${c3}#######${c1}#${c2}#####${c1}#${c3}#######".white().to_string(),
        "  ${c3}#####${c1}#######${c3}#####".white().to_string(),
    ]
}

fn freebsd_logo() -> Vec<String> {
    vec![
        "${c1}   _____ _ __   ___  ".red().to_string(),
        "${c1}  |  ___| '__| / _ \\ ".red().to_string(),
        "${c1}  | |_  | |   |  __/".red().to_string(),
        "${c1}  |  _| |_|    \\___|".red().to_string(),
        "${c1}  | |".red().to_string(),
        "${c1}  |_|${c2} ____   _____ _____".red().to_string(),
        "${c2}     |  _ \\ / ____|  __ \\".red().to_string(),
        "${c2}     | |_) | (___ | |  | |".red().to_string(),
        "${c2}     |  _ < \\___ \\| |  | |".red().to_string(),
        "${c2}     | |_) |____) | |__| |".red().to_string(),
        "${c2}     |____/|_____/|_____/".red().to_string(),
    ]
}

fn openbsd_logo() -> Vec<String> {
    vec![
        "                  ${c1}_".yellow().to_string(),
        "                 ${c1}(_)".yellow().to_string(),
        "       ${c1}  |    .".yellow().to_string(),
        "       ${c1}. |L  /|  .${c2} _".yellow().to_string(),
        "       ${c1}_ . |\\ _| . ${c2})".yellow().to_string(),
        "      ${c1}/ \\  || ${c1} Y |${c2} /".yellow().to_string(),
        "     ${c1}/ | \\   | ${c1})|${c2}/".yellow().to_string(),
        "    ${c1}Y   |   |${c1} /".yellow().to_string(),
        "    ${c1}|   |   ||".yellow().to_string(),
        "    ${c1}|   |   |;".yellow().to_string(),
        "    ${c1}|   |_  ;".yellow().to_string(),
        "    ${c1}L_J_L_J".yellow().to_string(),
    ]
}

fn android_logo() -> Vec<String> {
    vec![
        "         ${c1}--_".green().to_string(),
        "       ${c1}-///- _".green().to_string(),
        "     ${c1}////- -/".green().to_string(),
        "   ${c1}-////- -//".green().to_string(),
        "  ${c1}-////- .-///".green().to_string(),
        " ${c1}////- ${c2}(:::::)${c1} -///".green().to_string(),
        "${c1}////- ${c2}|${c3}*${c2}${c3}*${c2}|${c1} -///".green().to_string(),
        "${c1}///- ${c2}|${c3}*${c2}${c3}*${c2}| ${c1}-///".green().to_string(),
        "${c1}//- ${c2}(o${c1}${c2}o) ${c1}-///".green().to_string(),
        "${c1}/- ${c2}<_> ${c1}-///".green().to_string(),
        "${c1}- ${c2}/|\\ ${c1}-///".green().to_string(),
        " ${c1}/   \\ -//".green().to_string(),
        " ${c1}/     \\-/".green().to_string(),
    ]
}

fn gentoo_logo() -> Vec<String> {
    vec![
        "         ${c1}-/oyddmdhs+:.".magenta().to_string(),
        "     ${c1}-o${c2}dNMMMMMMMMNNmhy+${c1}-`".magenta().to_string(),
        "   ${c1}-y${c2}NMMMMMMMMMMMNNNmmdhy${c1}+-".magenta().to_string(),
        " ${c1}`o${c2}mMMMMMMMMMMMMNmdmmmmddhhy${c1}/`".magenta().to_string(),
        " ${c1}om${c2}MMMMMMMMMMMN${c1}hhyyyo${c2}hmdddhhhd${c1}o`".magenta().to_string(),
        "${c1}.y${c2}dMMMMMMMMMMd${c1}hs++so/s${c2}mdddhhhhdm${c1}+`".magenta().to_string(),
        " ${c1}oy${c2}hdmNMMMMMMMN${c1}dyooy${c2}dmddddhhhhyhN${c1}d.".magenta().to_string(),
        "  ${c1}:o${c2}yhhdNNMMMMMMMNNNmmdddhhhhhyym${c1}Mh".magenta().to_string(),
        "    ${c1}.${c2}+yhdddddmNMMMMMMNNNmmmmhhy${c1}NMMMy".magenta().to_string(),
        "      ${c1}+m${c2}Nmddddddhhdmmmmmmmmmy${c1}NMMMMh".magenta().to_string(),
        "     ${c1}`yNm${c2}mmmmmmddhdddddddmddmddddNMy".magenta().to_string(),
        "      ${c1}.dm${c2}mdddddddddddddddddmdddddmmNh".magenta().to_string(),
        "       ${c1}+m${c2}mdddddddddddddddddddddddh${c1}h:".magenta().to_string(),
        "        ${c1}`o${c2}hddddddddddddddddddddhhy${c1}.".magenta().to_string(),
    ]
}

fn manjaro_logo() -> Vec<String> {
    vec![
        "${c1}██████████████████  ████████".green().to_string(),
        "${c1}██████████████████  ████████".green().to_string(),
        "${c1}██████████████████  ████████".green().to_string(),
        "${c1}██████████████████  ████████".green().to_string(),
        "${c1}████████            ████████".green().to_string(),
        "${c1}████████  ████████  ████████".green().to_string(),
        "${c1}████████  ████████  ████████".green().to_string(),
        "${c1}████████  ████████  ████████".green().to_string(),
        "${c1}████████  ████████  ████████".green().to_string(),
        "${c1}████████  ████████  ████████".green().to_string(),
        "${c1}████████  ████████  ████████".green().to_string(),
        "${c1}████████  ████████  ████████".green().to_string(),
        "${c1}████████  ████████  ████████".green().to_string(),
    ]
}

fn mint_logo() -> Vec<String> {
    vec![
        "${c1}             ...-:::::-...".green().to_string(),
        "${c1}          .-MMMMMMMMMMMMMMM-.".green().to_string(),
        "${c1}      .-MMMM`..-:::::::-..`MMMM-.".green().to_string(),
        "${c1}    .:MMMM.:MMMMMMMMMMMMMMM:.MMMM:.".green().to_string(),
        "${c1}   -MMM-M---MMMMMMMMMMMMMMMMMMM.MMM-".green().to_string(),
        "${c1} `:MMM:MM`  :MMMM:....::-...-MMMM:MMM:`".green().to_string(),
        "${c1} :MMM:MMM`  :MM:`  ``    ``  `:MMM:MMM:".green().to_string(),
        "${c1}.MMM.MMMM`  :MM.  -MM.  .MM-  `MMMM.MMM.".green().to_string(),
        "${c1}:MMM:MMMM`  :MM.  -MM-  .MM:  `MMMM-MMM:".green().to_string(),
        "${c1}:MMM:MMMM`  :MM.  -MM-  .MM:  `MMMM:MMM:".green().to_string(),
        "${c1}:MMM:MMMM`  :MM.  -MM-  .MM:  `MMMM-MMM:".green().to_string(),
        "${c1}.MMM.MMMM`  :MM:--:MM:--:MM:  `MMMM.MMM.".green().to_string(),
        "${c1} :MMM:MMM-  `-MMMMMMMMMMMM-`  -MMM-MMM:".green().to_string(),
        "${c1}  :MMM:MMM:`                `:MMM:MMM:".green().to_string(),
        "${c1}   .MMM.MMMM:--------------:MMMM.MMM.".green().to_string(),
        "${c1}     '-MMMM.-MMMMMMMMMMMMMMM-.MMMM-'".green().to_string(),
        "${c1}       '.-MMMM``--:::::--``MMMM-.'".green().to_string(),
        "${c1}            '-MMMMMMMMMMMMM-'".green().to_string(),
        "${c1}               ``-------``".green().to_string(),
    ]
}

fn centos_logo() -> Vec<String> {
    vec![
        "                 ${c1}..".yellow().to_string(),
        "               ${c1}.PLTJ.".yellow().to_string(),
        "              ${c1}<><><><>".yellow().to_string(),
        "     ${c2}KKSSV' 4KKK ${c1}LJ${c4} KKKL.'VSSKK".yellow().to_string(),
        "     ${c2}KKV' 4KKKKK ${c1}LJ${c4} KKKKAL 'VKK".yellow().to_string(),
        "     ${c2}V' ' 'VKKKK ${c1}LJ${c4} KKKKV' ' 'V".yellow().to_string(),
        "     ${c2}.4MA.' 'VKK ${c1}LJ${c4} KKV' '.4Mb.".yellow().to_string(),
        "   ${c4}. ${c2}KKKKKA.' 'V ${c1}LJ${c4} V' '.4KKKKK ${c3}.".yellow().to_string(),
        " ${c4}.4D ${c2}KKKKKKKA.'' ${c1}LJ${c4} ''.4KKKKKKK ${c3}FA.".yellow().to_string(),
        "${c4}<QDD ++++++++++++  ${c3}++++++++++++ GFD>".yellow().to_string(),
        " ${c4}'VD ${c3}KKKKKKKK'.. ${c2}LJ ${c1}..'KKKKKKKK ${c3}FV".yellow().to_string(),
        "   ${c4}' ${c3}VKKKKK'. .4 ${c2}LJ ${c1}K. .'KKKKKV ${c3}'".yellow().to_string(),
        "      ${c3}'VK'. .4KK ${c2}LJ ${c1}KKA. .'KV'".yellow().to_string(),
        "     ${c3}A. . .4KKKK ${c2}LJ ${c1}KKKKA. . .4".yellow().to_string(),
        "     ${c3}KKA. 'KKKKK ${c2}LJ ${c1}KKKKK' .4KK".yellow().to_string(),
        "     ${c3}KKSSA. VKKK ${c2}LJ ${c1}KKKV .4SSKK".yellow().to_string(),
        "              ${c2}<><><><>".yellow().to_string(),
        "               ${c2}'MKKM'".yellow().to_string(),
        "                 ${c2}''".yellow().to_string(),
    ]
}

fn alpine_logo() -> Vec<String> {
    vec![
        "       ${c1}.hddddddddddddddddddddddh.".blue().to_string(),
        "      ${c1}:dddddddddddddddddddddddddd:".blue().to_string(),
        "     ${c1}/dddddddddddddddddddddddddddd/".blue().to_string(),
        "    ${c1}+dddddddddddddddddddddddddddddd+".blue().to_string(),
        "  ${c1}`sdddddddddddddddddddddddddddddddds`".blue().to_string(),
        "  ${c1}`ydddddddddddd++hdddddddddddddddddy`".blue().to_string(),
        "   ${c1}hddddddddddd+`  `+ddddh:-sdddddddddh".blue().to_string(),
        "   ${c1}hdddddddddd+`      `+y:    .sddddddh".blue().to_string(),
        "   ${c1}ddddddddh+`   `//`   `.`     -sddddd".blue().to_string(),
        "   ${c1}ddddddh+`   `/hddh/`   `:s-    -sddd".blue().to_string(),
        "   ${c1}ddddh+`   `/+/dddddh/`   `+s-    -sd".blue().to_string(),
        "   ${c1}ddd+`   `/o` :dddddddh/`   `oy-    .".blue().to_string(),
        "   ${c1}hdddyo+ohddyosdddddddddho+oydddy++oh".blue().to_string(),
        "   ${c1}.hddddddddddddddddddddddddddddddddh.".blue().to_string(),
        "     ${c1}`yddddddddddddddddddddddddddddy`".blue().to_string(),
        "       ${c1}`sdddddddddddddddddddddddds`".blue().to_string(),
        "          ${c1}+dddddddddddddddddd+".blue().to_string(),
        "             ${c1}:hddddddddddh:".blue().to_string(),
        "                ${c1}`.::-::.`".blue().to_string(),
    ]
}

fn nixos_logo() -> Vec<String> {
    vec![
        "          ${c1}▗▄▄▄       ${c2}▗▄▄▄▄    ▄▄▄▖".blue().to_string(),
        "          ${c1}▜███▙       ${c2}▜███▙  ▟███▛".blue().to_string(),
        "           ${c1}▜███▙       ${c2}▜███▙▟███▛".blue().to_string(),
        "            ${c1}▜███▙       ${c2}▜██████▛".blue().to_string(),
        "     ${c1}▟█████████████████▙ ${c2}▜████▛     ${c1}▟▙".blue().to_string(),
        "    ${c1}▟███████████████████▙ ${c2}▜███▙    ${c1}▟██▙".blue().to_string(),
        "           ${c2}▄▄▄▄▖           ▜███▙  ${c1}▟███▛".blue().to_string(),
        "          ${c2}▟███▛             ▜██▛ ${c1}▟███▛".blue().to_string(),
        "         ${c2}▟███▛               ▜▛ ${c1}▟███▛".blue().to_string(),
        "${c2}▟███████████▙      ▟█████████████████▛".blue().to_string(),
        "${c2}▜██████████▛      ▟███████████████▛".blue().to_string(),
        "      ${c2}▟███▙       ▟██▛${c1}▜███▙".blue().to_string(),
        "     ${c2}▟███▛       ▟██▛  ${c1}▜███▙ ${c2}▟████████████▙".blue().to_string(),
        "    ${c2}▟███▛       ${c1}▟██▛    ▜███▙ ${c2}▜██████████▛".blue().to_string(),
        "   ${c2}▟███▛       ${c1}▟███▙     ▜███▙".blue().to_string(),
        "  ${c2}▟███▛       ${c1}▟███████▙    ▜███▙".blue().to_string(),
    ]
}

fn opensuse_logo() -> Vec<String> {
    vec![
        "           ${c1}.;ldkO0000Okdl;.".green().to_string(),
        "       ${c1}.;d00xl:^''''''^:ok00d;.".green().to_string(),
        "     ${c1}.d00l'                'o00d.".green().to_string(),
        "   ${c1}.d0Kd'${c2}  Okxol:;,.          ${c1}:O0d.".green().to_string(),
        "  ${c1}.OKKKK0k${c2}OKKKKKKKKKKOxo:,      ${c1}lKO.".green().to_string(),
        " ${c1},0KKKKKKKKKKKKKKKK0P^${c2},,,,^dx:${c1}    ;00,".green().to_string(),
        "${c1}.OKKKKKKKKKKKKKKKKk'${c2}.oOPPb.${c1}'0k.${c2}   cKO.".green().to_string(),
        "${c1}:KKKKKKKKKKKKKKKKK: ${c2}kKx..dd ${c1}lKd${c2}   'OK:".green().to_string(),
        "${c1}dKKKKKKKKKKKOx0KKKd ${c2}^0KKKO' ${c1}kKKc${c2}   dKd".green().to_string(),
        "${c1}dKKKKKKKKKKKK;.;oOKx,..${c2}^${c1}..;kKKK0.${c2}  dKd".green().to_string(),
        "${c1}:KKKKKKKKKKKK0o;...^cdxxOK0O/^^'  ${c2}.0K:".green().to_string(),
        " ${c1}kKKKKKKKKKKKKKKKK0x;,,......,;od  ${c2}lKk".green().to_string(),
        " ${c1}'0KKKKKKKKKKKKKKKKKKKKK00KKOo^  ${c2}c00'".green().to_string(),
        "  ${c1}'kKKKOxddxkOO00000Okxoc;''   ${c2}.dKk'".green().to_string(),
        "    ${c1}l0Ko.                    .c00l'".green().to_string(),
        "     ${c1}'l0Kk:.              .;xK0l'".green().to_string(),
        "        ${c1}'lkK0xl:;,,,,;:ldO0kl'".green().to_string(),
        "            ${c1}'^:ldxkkkkxdl:^'".green().to_string(),
    ]
}
