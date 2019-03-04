use rusqlite::Rows;
use rusqlite::types::Type;
use rusqlite::types::Value;
use horrorshow::prelude::*;
use horrorshow::helper::doctype;

use tables::WmiOsVersion;

pub fn map(values: &mut Rows) -> Vec<Vec<String>> {
    let mut table: Vec<Vec<String>> = Vec::new();
    let mut row: Vec<String> = Vec::new();
    loop {
        if let Some(v) = values.next() {
            if let Some(res) = v.ok() {
                for i in 0..res.column_count() {
                    let val = Value::data_type(&res.get(i));
                    match val {
                        Type::Real | Type::Integer => {
                            row.push(res.get::<usize, i64>(i).to_string());
                        },
                        Type::Text => {
                            row.push(res.get::<usize, String>(i))
                        },
                        _ => {
                            // Do nothing.
                        }
                    }
                }
                table.push(row);
                row = Vec::new();
            }
        } else {
            break
        }
    }
    table
}

pub fn print_html(columns: Vec<String>, values: &mut Rows) {
    let map = map(values);
    let actual = format!(
        "{}",
        html! {
            : doctype::HTML;
            html {
                head {
                    title : "title";
                    style(type="text/css") : r#"
                body {
                background: white;
                padding: 0;
                margin: 0;
            }
        @media print {
            body { background: transparent; }
    }
    td { vertical-align: top; }
    td.value {
        /*min-width: 217px;*/
        vertical-align: top;
    }
    pre {
        padding: 0px;
        margin: 0;
        white-space: pre-wrap;
        font-family: segoe ui, arial;
        font-size: 11px;
    }
        .HeaderHolder {
        padding: 9px;
        width: 725px;
    }
        .HeaderTitle {
        color: #454545;
        padding: 2px 8px;
    }
        .Information {
        padding: 0 9px 9px;
        width: 700px;
    }
        .InformationInfos {
        color: #454545;
        padding: 2px 8px;
    }
    table { width: 100% }
        .InformationType {
        margin: 0 3px;
        padding: 8px 0 0 34px;
        position: relative;
    }
        .InformationType .Container {
        background-color: #F5F5F5;
        border: 1px solid silver;
        padding: 5px 10px;
        width: 97%;
        border-radius: 0 3px 3px 0;
        -moz-border-radius: 0 3px 3px 0;
        -webkit-border-radius: 0 3px 3px 0;
        /* min-height ie fix */
        height: auto !important;
        height: 26px;
        min-height: 26px;
        overflow: hidden;
        display: -webkit-box;
        display: -webkit-flex;
        display: -ms-flexbox;
        display: flex;
        -webkit-flex-wrap: wrap;
        -ms-flex-wrap: wrap;
        flex-wrap: wrap;
    }
        .InstallDate
    {
        white-space: nowrap;
    }
        .HeaderHolder fieldset table {
        background-color: #F8F8F8;
    padding: 10px;
    width: 100%;
    margin: 10px 3px;
}
.HeaderHolder .Drive table, .HeaderHolder .LocalAccount table, .HeaderHolder .NetworkAdapter table {
float: left;
margin: 0 6px 6px 0;
padding: 3px 6px;
width: 49%;
border: 1px solid transparent;
}
.Container table.box {
border: 1px solid lightgray;
display: -webkit-box;
display: -webkit-flex;
display: -ms-flexbox;
display: flex;
-webkit-box-flex: 0;
-webkit-flex: 0 0 46%;
-ms-flex: 0 0 46%;
flex: 0 0 46%;
max-width: 50%;
}
fieldset {
border: 1px solid #B8B8B8;
border-radius: 0 3px 3px;
-moz-border-radius: 0 3px 3px;
-webkit-border-radius: 0 3px 3px;
margin-bottom: 4px;
margin-top: 20px;
padding: 5px 3px 3px 5px;
position: relative;
background: #EBEBEB;
}
fieldset legend {
background: #EBEBEB;
border: 1px solid #B8B8B8;
border-radius: 3px 3px 0 0;
-moz-border-radius: 3px 3px 0 0;
-webkit-border-radius: 3px 3px 0 0;
display: inline-block;
font-family: arial;
font-size: 11px;
font-weight: bold;
margin: 0;
padding: 2px 5px 0;
position: absolute;
top: -17px;
left: -1px;
border-bottom: none;
}
th, .label {
font-family: segoe ui, arial;
font-size: 11px;
font-weight: normal;
line-height: 16px;
padding: 0 5px 0 0;
white-space: nowrap;
}
th { text-align: left; }
.title {
color: #5E5E5E;
font-family: segoe ui, arial;
font-size: 16px;
font-weight: bold;
line-height: 24px;
width: 100%;
}
.title2 {
color: #808080;
font-family: segoe ui, arial;
font-size: 14px;
font-weight: bold;
line-height: 24px;
width: 100%;
}
.label {
color: #808080;
/*width: 105px;*/
}
.label.EmailPOP3Port, .label.EmailIMAPPort, .label.EmailSMTPPort { width: 30px; }
.value, td {
empty-cells: show;
font-family: segoe ui, arial;
font-size: 11px;
line-height: 16px;
padding: 0 2px;
}
.HeaderTitle .value.Name {
font-family: "Century Gothic", "Helvetica", "Lucida Grande", "Lucida Sans Unicode", arial, sans-serif;
font-size: 18px;
font-weight: bold;
line-height: 20px;
padding: 0;
}
.value.DescriptionRtf, .value.DescriptionText, .value.DescriptionUrl {
border: 1px outset lightgray;
padding: 2px;
}
a.encrypted {
color: Salmon;
font-family: Times New Roman;
font-size: 17px;
text-decoration: none;
}
a.encrypted:hover {
text-decoration: underline;
CURSOR: pointer;
CURSOR: hand;
}
a.encrypted:after {
content: url("data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAABGdBTUEAAK/INwWK6QAAABl0RVh0U29mdHdhcmUAQWRvYmUgSW1hZ2VSZWFkeXHJZTwAAALySURBVDhPZZJbSJNxGMYNkg5IEBFEF3lRF50uLOhwURcVCNGZDpQF1YU3Eh0oRUFXs4GHTCcz5xaahxkzcWRHm+ZormW2PB8qD3Oa1kg3zQ77dPbrv49qZRcP73fxvb/ned/3HwKE/K3S0tKwkpISZWFhoTk/P9+h1+sdWq3WnJOTo1Sr1WEz//+n2WAwRBQXF/fV1NTQ0NCAw+GQq9VqpaKigoyMjL60tLSIvyF/AKI5rKioqNtms1FbW4v4Jjc3F41Gg0iAyWSSpVKpupVK5bzfkD8A0RBrNpuxWCzk5eX1isjh2dnZoZmZmaHp6enhKSkpQ0ajkYKCApKSkmL/A4h5TQH3gLNwXTVz1uTk5FUiPmVlZSQkJJj+A+h0ujq73Y5wJisra+5MgEKhmCucKS8vJy4uru4fgKvuNMbCVERkRGR6nkThrDpM38MD9FbuoadiJ13GSOLj4wPNXFdE80a3TjAIkXfgsp7CP/GS6YlX/PjWDt+64Msb8PULOeHrO/zjr5kctTL56amsTs2aIKDfckIA6pGcsUwPp8GHGzCkB88dGDXCoA6p4xITjmOM1+9j0l1Nx/UVQYCz+qhweIHUc47R6iN4n54WkJsM3N6Ay7ALBtS4xRiDhs2M2XYifayiPXVZEND3+CD+MTu+tzFMDyTCRzW4dThvrefetUMCkI7UHMP48/2MPYtE+vCItqtLgoDee3uZ8tbh64xm5NFukSAKPt2gVxtBefIecKlwl23DJYDe2u1IQw9oVSwKAgJbnvI843vbSfzdZ8T8l8XsWfBZK6oGnIn4HMdFcyQe8xakwfu0JCwIArqNO5gasfC16Rgjd7fhNe/Fa43Caz+F13YCr0g1XLoJl341nqrN+AYraY4NvOZfZ3xXslWcpga/0A+PRZzQLs7XCJNtojaLJC/wu81I7+/jG7grq+lCaBDwNn+j/DC6ctbSqV5JR8ZyecttqqW0XllMS+JCOXJz3HyaLs6h6fxsGs/OkgE/AUhP8wdgnhz4AAAAAElFTkSuQmCC");
margin-left: 2px;
vertical-align: text-top;
}
.ComputerIcon{
display: block;
height: 36px;
width: 36px;
left: -2px;
position: absolute;
border-radius: 3px 0 0 3px;
-moz-border-radius: 3px 0 0 3px;
-webkit-border-radius: 3px 0 0 3px;
top: 8px;
border: 1px solid silver;
border-right: none;
background-color: #F5F5F5;
background-position: 6px 5px;
background-repeat: no-repeat;
background-size: 24px auto;
background-image:url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAYAAADgdz34AAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAADsMAAA7DAcdvqGQAAANySURBVEhL7VVdSFNhGN5FF/0QhGWUkv0X7pxmFhRZIGbR751ERRb9QCiezaaZJVg3ZZBBEAQR3XTXz0W4nbO59mOp8y/tj2ZFtc25rB3XNnW6tPLp/daBkrIuVhdBDzycw/dxnuf93vf93qP66+B1xq28/s5JXmv6s9TbTjJtFaeV/JknWpF5vAUrKh/+ETItpsm0VbxWHNDoreAEcZB4lxckR0LUSnWcVowyTaat4gQpnFFWD7XWVK5kLWFwxWJZxtF7FLQUVgwawBeb1yj7CSO9WFzNgh5jsExnylX2EwalKOe/wS/xbxmMjo5OAjCNkb2ztYm7b6xJ2IAEZ5HgkY7IcINNHpStgUG5PfyhkdYOlV9qy1+itY81oPmRrXz7W5CIxtU/3HzWN4J1zb1ItfqRYunGaqeMKnc/6n2RnvzzLVhwuCakGLDjiIeU738JEk92DQw3H3geQ5LoxkzxNQoeBNDyPoYdLW8xqeYV9j0JQ3oRxpZK+1cDTYkDNEd8vGDexgr0UxaKOSv0tTkdnlD12a4YZpB4msmNqSSoeySje+gjiumZbHyN6cQzniguGl6QARtMpQ5oSuxgJxmPXMld5FQ6cdMVRlaDH7Mp8pX2LlQ8DeLCyzBOdQaxqNaDBcQ5ZJzlDOD2s2CITdMrmiPWEI3WEDvNT+jnBKN3SYHBu7nC5jN4I5/TLF7MIoNNZOSnyBmoyGTqjhswplFdLO+iIVX2qboJan1t0rgUpPUavSNvaYEhL/9cwz5Lz0DfXKsP80lkntmD7c438fRc6+pHijTWwCYPyUrpxodaMDqWlzciXWtG9nEbrj8OIKveH09DMp2ikArc//Ez9rS9RSoZLCRxtre2SUZ7ONakyIwPtSBa4jUQTFAXiai+3YnTryJIUop52R2Jp6imJwrO6kWa2Y3plKoq3zD6Rj6VKjI/gqVOU3pvPbXvA9YEdE+wuNCITZV2iJ292PuoF5Opgw52BFAnD+Kqpw+LLR5MI9P9z4ZA96SVWjpVkfsGvkjamFHWWM0LojGz4j7Yn4l1GGtlZrRIsGLvhXaYXkZR5Y1hVWMAM2q7MZOY1dRLkY/A1fehjcQzFMmx0OiMfOYx5y6uSMpbJogb2Aj5nuk6Q+6UnbdyL99y5ZJIyf1QzHnnXTRIHROkUeGktVLi7K9qKtUXxQRXeUIsKhEAAAAASUVORK5CYII=);}
.NetworkAdapterIcon{
display: block;
height: 36px;
width: 36px;
left: -2px;
position: absolute;
border-radius: 3px 0 0 3px;
-moz-border-radius: 3px 0 0 3px;
-webkit-border-radius: 3px 0 0 3px;
top: 8px;
border: 1px solid silver;
border-right: none;
background-color: #F5F5F5;
background-position: 6px 5px;
background-repeat: no-repeat;
background-size: 24px auto;
background-image:url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAYAAADgdz34AAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAADsMAAA7DAcdvqGQAAAMwSURBVEhL3VTNTxNBFF+8evAjBDyTSGR3ZnahGhNjoiJRCUnFaIh6NSHQ7kc/aEGx4Ef8OvkPeNIYE5Sg7exHC22pRRpEEIwevHAy8QqJEkOi65ttCyUCVtOLvuRldt9v5r03v/fecP++CN20ASlGGqvWtKDQvKrGNAkkJ3mv6WZ7kEzvY18iu4r/RolvNIcU/QFSx2rhsNkGh5eQrM8KMp0RZP2NoOgzpCfziVf0m04ARV9BWnwR8FlwMMPwTRX2INVcEEMvbaREj3C8Zl4A4A5zVCpIHmkXvHk71uI2BL7uAGUIrxqHxfCELSjWMQ7oOF90hBSaJKHsayRb+xpkvRWyuc3sWDVt2Od8c5xdtbXCfpm2sBsIXvNoPoBM7zIAAjwTQ1lLUhN7gfe2dQE2uOVmglXj+LoAUOR7DOBl6ifh7GXek96DFXq6YgGKjiDrJal30kbexH7BGztZkQC8ql8UvPRGAVsV3jN8aq3Ils3LsYgDlCFYjbuA6nwArJluaNNXQNOAoBj9kPVVpOoREkg9hQDX2AHAviPNnIIa3SpP9SdiOAtJ6c2cCK0kQUtJvTlbgqhiMG1LfTnbFXkLGdD+fACaxL7Rb+wmpQq0fYWEvrCV0Vi0I836AUkuYH8CO3yRnnFmXCGycYldCzL43HhlijlwbkCCD7fzfmt3qZKu4RoMk078o2dZQ5CueE0pXtc5tIMbHNyWL0hPxkaqtQzPxiGkvqgV5NjHxv43MFyxrYarCppiUQzDxEJzOM42EuD9BKOETR48F8ssewn4cygqdFFRbDs/SKI2slPQzEfOcwDUOKtiPmZ2Z2Op8IrBg6MEUo0UrJPA9xTwlwLOU4CdK2xbJ8hDO0goMwFn5tgzwlaJ/Su0o7ClMiJ0Rw+w2xPP84MFU2WlAYrrGpiHG8TOFEy/Cu6ju0gw2cx69k8UB5JuqNd7HEglocgfcGjcXYozn8w3e4pbm6CgzhzAM1GuugbfQWFpliUJT3mG/a/hObspMgczEW/lWM9iX7oFyyYoLVvFwFg7NMU8CaaG2CoG0u1rOPgCn8y3Q9PfSn1ntBq6ra4+EK0umP474bif7rdNsYHz3JkAAAAASUVORK5CYII=);}
.DriveIcon{
display: block;
height: 36px;
width: 36px;
left: -2px;
position: absolute;
border-radius: 3px 0 0 3px;
-moz-border-radius: 3px 0 0 3px;
-webkit-border-radius: 3px 0 0 3px;
top: 8px;
border: 1px solid silver;
border-right: none;
background-color: #F5F5F5;
background-position: 6px 5px;
background-repeat: no-repeat;
background-size: 24px auto;
background-image:url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAYAAADgdz34AAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAADsMAAA7DAcdvqGQAAAOLSURBVEhLtVbNS1RRFLcoWkRhVKsgIUydefe9GRVaFBX1ByRBi6CEIOhj5t33nPEzMqQ2rQpaRARR1LKi0nffG8fRbDDLiGrTokUimasso0wyqHn9zn1XHRv7MjtweHfuued3vu+donwqS3asYaZIG4meMY27f8VSJ+5kotxbq+AKKWQ6W3U7nTMS3b5e1xVwIuMbyR7fqL8L7g04iXX+GbD8bad9xr3tCq6QNFM0kjLjruQpEGa6n/F7FNG91Ex3BOuP4BwZ1u3OmfPQDXOnWcHNpu1tvUsA1EkeMyslvYY3QzB6JmSLbdVI3yburYzYt4tDlrMRxg4jNRlmeYgYOlzIaHG+q/rQxaUKdobCVsd6jYu3RoLCpYPurTD3wko8J5FTAIwhilGZKqQIGO9CMVGijswQ484+hPmNcqxz4ZK3SvRb0mIdtTpPfZHp5CKH6PYr0QwB/BKBM56aiHKxWW1Pk94iVmlm+xb9yJ1ytTWLwqZzk2pmJLspistqO6DyxjsrEOrTaMtDSk02YvcWK5Ekaj0o3Q46JjOCvO9WomnSTXGA6haBk6jNs9L8DEB5B4DfR5ru++hlz6hPL1ciSczsrIADr6ONfX60+YGvm6lWJZom1GuP7Co4QViG6e5UIhiw3CaZe4SIThqKJJx1tG9YDtPjqV3kTbiufafGvQvghop4ZrVUzCN4fZpShOjkzKADg3aF8jJmiRuRhiw2XdUJ7hWsz8PYMIvjYFvbYnn4J1QRd8twfjgosutLLFPcJGwpREivpoTE5EHlsYHCYs1B4Vh7Kbztkd5zL3AywBom7CK0Vw0KJ0ObMkCDJifU8q6GYyI6q2CS/EX6UbEBdTmIaF/gHvpBH2miWwDYRSzung2sK2EeG2SYi3GsOwF0TjOdk5RrpO4awF/Q5ErnlOezdOkmADaK4wxSZ0Qa7s3J0aY+pOuRX9X6xK868Qzfp37l8ceymyjXc+kQR5v7kWJnkFr0OrO7euDpwjIwCVvl9D8SrNSiBhnkdkGYvMesZAJMUUtvwJtf1eCPuR55x1VT2TKALxiYhI1rQHzIn4H5ssSw3H7cCjaAr8vWxcNEIz72rwZ0CzNjpyfxDtRQ2ivrRQlmCW+1+LQgBtRg5vCOnIokutdplrcXe18hkxFMyIsORubPaTm5AMyBnxMmzQh96aHJsrpMwd+Q+TDVEzyJThpndV1jjLvZ70nEuefWxXQoAAAAAElFTkSuQmCC);}
.LocalAccountIcon{
display: block;
height: 36px;
width: 36px;
left: -2px;
position: absolute;
border-radius: 3px 0 0 3px;
-moz-border-radius: 3px 0 0 3px;
-webkit-border-radius: 3px 0 0 3px;
top: 8px;
border: 1px solid silver;
border-right: none;
background-color: #F5F5F5;
background-position: 6px 5px;
background-repeat: no-repeat;
background-size: 24px auto;
background-image:url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAYAAADgdz34AAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAADsMAAA7DAcdvqGQAAALWSURBVEhLpVZNaBNBFE5BoWIVPBWhtRehJjOzaSkFb/Wk6EW8ePDSgzTaZH/y01S8KHizePGmh9qT+AP+JNndbNrY+FMQqZ6LgodivGkORamgsn5vM6YaN002fvBYmJk338x733uzoXZQNHuU684CU60q0+0a08xVrlqqkintlku6B0sUj/Bk6UN05qkrkkuwRVdJlV0lveyCcH4gdW+XXBoch6Ye7GeavRqdeeZyzfrLhFFyFZAx1YzL5cERjpsnlfQTl+v2PwRkSqaCr7kyFnu9U7oEg1CLWQoF11oQgBxhesfSzqB0CQaWKBj1U7YiQB40a43r5X7pEgxcy09ww/kuDMefAORCNQuhkNsjXYIhEq/08YR5dyT7HEklEnkT5ESkEB7N2sQtj8nl3YFNPxrkerEIqf6gkNCpldQSkXz6LwU1oUfozhmc/g6KzMZNrobjOSbnuofQcoex4ZVounINsW6clhlmWEmX51DdcxHVmjqo2XvlVOdAgR3FiT+OzK64oxdfUUF9Q5sY9+ZUc57GaE7JLFOibw1nH+/xHDvBxOXKDmyyGJ19UU8skkqxx1h2aHKhl6vmOrUMSjh9BVpH2LCOS/f2wJUHmFZ461VxQ5IoOEgSdg62ifBszSH5Ea1wQbq3h9DNMXTLqiC1yE2gJBBYn1G570HwszFOBLgBJHtdurcHQnKCqYUvIlnaIiAj/fsUHckWxA/HYjc760lMtc8KAyemUzdt5meUB+TnTX+n7wMSe4lU8juR2xndiNo5CDbEtLlPbrE9eDx3WkmW1+j1orD4bewZ5hD7ryLprEcSZj7QCxeJ5Q8wHd0UCfTdHPKlRwe5us31nBKJ3++Trp2D6flxyoXwywWNeVINIM9m0GMPdWz4KadBirqQy4NDnC8OI8ZVr+9Txf5h9aeUbmGfksuDY2iy0ouedAM6r9V/V6yGcaNUg3JeClS9XN4CodAvEVI2K5LovsQAAAAASUVORK5CYII=);}
.ProductIcon{
display: block;
height: 36px;
width: 36px;
left: -2px;
position: absolute;
border-radius: 3px 0 0 3px;
-moz-border-radius: 3px 0 0 3px;
-webkit-border-radius: 3px 0 0 3px;
top: 8px;
border: 1px solid silver;
border-right: none;
background-color: #F5F5F5;
background-position: 6px 5px;
background-repeat: no-repeat;
background-size: 24px auto;
background-image:url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAYAAADgdz34AAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAADsMAAA7DAcdvqGQAAAPmSURBVEhLrVZNj9xEEHUixIUjX38AhYndbc8lITeunMORSBwSod213Z7ZzWYFQgwHrmGlRIgfAGIACcTY3bY3BJLsZ5AQVw4oOW1WIYdB2iwrLol5rz3ZzMdumAMllWR1d72qelVdbedI6VTHZdw7I5LifRGZwgv1XS82/Vr1XRHp0lPFBzLOz/DswGo6caPstJeYL7w4/9tf+LmS7R8r2bo2qljjHs7si9h0G3Q0jXhxNitUuRMsrlYyWalg/Ey1ji7eqkRS9qXK5wYwh8oxL9IfSlU89tvXDwUbV3/+J2RgfvdUfl5GZiNYuFHJMPuYWDXkkJwM0xmpykdM/zAwqyqvFd/Mzm9d6/uRPkt7cb73qoh1WTtJQwv6RDyVnvKU+fOoyOnUn0ctktKqD5Bg8VblRdkPjlMdRHsq/P5FNMUvsHl4MtFv2EW38+3zoOZLGowDizi3wIj6nheZb9woXUb3LPMbnN+TSVG5yiz7CysvWDCIq7J3rXOF88B26AlF3Z0oKKhgRnBuAHoO3dLykuIKzl2RKm1zDWeK5tJ6RacNRI96XADWbUuzKvZsFiwsUx4Bh1pKYmOwfxZAq81L66BlzSq/RVyseWH6NuqmZev6I9D1m1B14WkfsIWB7bixKdlqw+CDQu8Ipc/hTmw2lzbZLZuIVNVqNppLG6zBlpjtMbv7waW1Ch34FAOYxEbfmzvjncPoyTNuaasJQ3xv+TPfvTKg2WnG+cuIbpOZSKVJV9fWagiDmMSmg/6IA3CPtLn5qYizz5qgREaTFwhRzzZxGZHF59DLtKHtmIP+dA5m9YQDEZqZ2oH+TweTFKFALilC+qQBIFuvgZYBtnPivfQlWwdb7HTefRZF40WWrRV0CiMzD3AP3nHDbItFxve6CHUM4Ajn1gZFvs0ziHxbtkeDfFpk26a1d9uukdlFRCkc/SNaZc5WZEvWbbpq1fa+KtZf515k0vHoqQdt2sDMx1zfqy9a3nPn9FukQUZ60TpMioJRYlK2Zau4CsdXSYuNPNKZzX6IeyqxcMsfEtuOCgypr3mBEO0FSzLkzc6N50TYi3F59sDnNgy7iOgy1Y11Fw6268hHwakBxzfqYkcFhQ8MotgF0B+N0Jywi5RO5zj2Mhowm4NhB+DxxniidmAq88BtZ6cHKLV4UToXXLzJ2/iriNKGXUOvA2gffNcApGKMjmGVCV66pHyMLA99eI6hSz4KFm6ivfI7mJKfIM0dOe3jg3PMDjad4RE+IXid5jAR/wrQhtOAs6D181rcx3AcfWiOEv4pwLiLi7JvuUenkPMRxRr3kC0f/a+mfvQP5H/5bXGcfwEwXuFcHHTSDwAAAABJRU5ErkJggg==);}
.QuickFixEngineeringIcon{
display: block;
height: 36px;
width: 36px;
left: -2px;
position: absolute;
border-radius: 3px 0 0 3px;
-moz-border-radius: 3px 0 0 3px;
-webkit-border-radius: 3px 0 0 3px;
top: 8px;
border: 1px solid silver;
border-right: none;
background-color: #F5F5F5;
background-position: 6px 5px;
background-repeat: no-repeat;
background-size: 24px auto;
background-image:url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAYAAADgdz34AAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAADsMAAA7DAcdvqGQAAAPmSURBVEhLrVZNj9xEEHUixIUjX38AhYndbc8lITeunMORSBwSod213Z7ZzWYFQgwHrmGlRIgfAGIACcTY3bY3BJLsZ5AQVw4oOW1WIYdB2iwrLol5rz3ZzMdumAMllWR1d72qelVdbedI6VTHZdw7I5LifRGZwgv1XS82/Vr1XRHp0lPFBzLOz/DswGo6caPstJeYL7w4/9tf+LmS7R8r2bo2qljjHs7si9h0G3Q0jXhxNitUuRMsrlYyWalg/Ey1ji7eqkRS9qXK5wYwh8oxL9IfSlU89tvXDwUbV3/+J2RgfvdUfl5GZiNYuFHJMPuYWDXkkJwM0xmpykdM/zAwqyqvFd/Mzm9d6/uRPkt7cb73qoh1WTtJQwv6RDyVnvKU+fOoyOnUn0ctktKqD5Bg8VblRdkPjlMdRHsq/P5FNMUvsHl4MtFv2EW38+3zoOZLGowDizi3wIj6nheZb9woXUb3LPMbnN+TSVG5yiz7CysvWDCIq7J3rXOF88B26AlF3Z0oKKhgRnBuAHoO3dLykuIKzl2RKm1zDWeK5tJ6RacNRI96XADWbUuzKvZsFiwsUx4Bh1pKYmOwfxZAq81L66BlzSq/RVyseWH6NuqmZev6I9D1m1B14WkfsIWB7bixKdlqw+CDQu8Ipc/hTmw2lzbZLZuIVNVqNppLG6zBlpjtMbv7waW1Ch34FAOYxEbfmzvjncPoyTNuaasJQ3xv+TPfvTKg2WnG+cuIbpOZSKVJV9fWagiDmMSmg/6IA3CPtLn5qYizz5qgREaTFwhRzzZxGZHF59DLtKHtmIP+dA5m9YQDEZqZ2oH+TweTFKFALilC+qQBIFuvgZYBtnPivfQlWwdb7HTefRZF40WWrRV0CiMzD3AP3nHDbItFxve6CHUM4Ajn1gZFvs0ziHxbtkeDfFpk26a1d9uukdlFRCkc/SNaZc5WZEvWbbpq1fa+KtZf515k0vHoqQdt2sDMx1zfqy9a3nPn9FukQUZ60TpMioJRYlK2Zau4CsdXSYuNPNKZzX6IeyqxcMsfEtuOCgypr3mBEO0FSzLkzc6N50TYi3F59sDnNgy7iOgy1Y11Fw6268hHwakBxzfqYkcFhQ8MotgF0B+N0Jywi5RO5zj2Mhowm4NhB+DxxniidmAq88BtZ6cHKLV4UToXXLzJ2/iriNKGXUOvA2gffNcApGKMjmGVCV66pHyMLA99eI6hSz4KFm6ivfI7mJKfIM0dOe3jg3PMDjad4RE+IXid5jAR/wrQhtOAs6D181rcx3AcfWiOEv4pwLiLi7JvuUenkPMRxRr3kC0f/a+mfvQP5H/5bXGcfwEwXuFcHHTSDwAAAABJRU5ErkJggg==);}
.PrinterIcon{
display: block;
height: 36px;
width: 36px;
left: -2px;
position: absolute;
border-radius: 3px 0 0 3px;
-moz-border-radius: 3px 0 0 3px;
-webkit-border-radius: 3px 0 0 3px;
top: 8px;
border: 1px solid silver;
border-right: none;
background-color: #F5F5F5;
background-position: 6px 5px;
background-repeat: no-repeat;
background-size: 24px auto;
background-image:url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAYAAADgdz34AAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAADsMAAA7DAcdvqGQAAAK3SURBVEhLvZVNbxJBGMe5ePNgP4AHj+4r4FVPXvwCxmMTU00Ks9DyDmJjjIkH48cwsR60ZmcWKC1QSrUa9QPYF716qDGx0YvJ+n+GXVtYhMU2TvIPsDzP7z/zzDOzkTDj0u13ZwzWuBDLb13VU85IqfP18174dENL2DcVxkuKxZfi1feukVkfVLbtRotbrsqcJ15K+KEzft3ItH5oTNQ0ZmfNYs/VLCcgM7/hakne89LCDZWJG0j+FittY3ZiWRoUNgEUAZm5jotJtLzUycNI8GtI/G5kO1h+z1Ut8UxN2ulTM1ASzmXdqs0pKXtWX2jOXmT2FazggZk/JQOV2ctGpv0ZM/+C74eaxX/ShkYLPZdWcVzG4pprYqOnNdigjlEZ76D+FTXJ72A/ivjMqdagNMYbsfL23w3M9Itz+jyf0UubM0qifZaeAbxGtcfMdwBxxgngjzLWMyAGsYhJ7AjKsKenVw9QxwMErUgDSzSpHJjdLmpcn6AdGWvxdZnLxHNiEZPYgPFD/7AguOsb9JfNG9SeYwUTGXtk0CEWMYlNsK/6wmq/E/wgS7yKV9660dJrN179MFYyBrHIfePlNolFTGIfGdBptHhdBjFRMHPdp9jYUPJii30D7hArYGAsNunBPgKXyGBkOYYEWA4lKmtJu4zGqKLL7qMD94gVMEC9XT3dkL0eHer1UZJngkqRqvWVruPya0mwZAUMRpzQcTq+Z/5AF97td5Q4uQGVAcBdzLbkC6wmnXj6/8QGgyXtS8LxnP4PGoxI+BcRQ7KGDTyTT/h9T3YEsyvyHgolikUO5Vp83+f5Br/o5NHlhk2b7q00YqBVu8SSr1GwI6jXSy3daOFot/CKfExBCi4uatU/LThBFEs3L+VqKfGIWMQkNj0LDDUp5sx85yFmEEoUqzNxy0v/nyMS+Q1B04An/QgQFQAAAABJRU5ErkJggg==);}
.ProcessorIcon{
display: block;
height: 36px;
width: 36px;
left: -2px;
position: absolute;
border-radius: 3px 0 0 3px;
-moz-border-radius: 3px 0 0 3px;
-webkit-border-radius: 3px 0 0 3px;
top: 8px;
border: 1px solid silver;
border-right: none;
background-color: #F5F5F5;
background-position: 6px 5px;
background-repeat: no-repeat;
background-size: 24px auto;
background-image:url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAYAAADgdz34AAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAADsMAAA7DAcdvqGQAAAMcSURBVEhLtVZdTxNBFN2/gJrooxgIdHdnKRD5AYhGjQL6om8aFVT2o9uWAkkpwTfCPzFBBHZmt5QAGoJgEKIxxvhjYL1nO7ZbKLWK3uQkvXfvPTNzZuZOldOMveQt3bnta2q6eE6GFNXy+gDpKvh2PKdp001xvbfwOWQ2H5IhRbP5d8IP6ZLv30WOYfObMtS8JaxgoCe/HxoWvy9Dimbxr4Rv0lV0a2UYOTTQDRlq3jrNxfO648+rY/ySDJFE/LFue0+kq+jO8kXk9I6uXJCh5kwdK7ZppufqTjFLkuR1258BNEtMABXf9vPlHM9V08ttsryxsRdBB5HsJHNbYXJym/AhTE7tlIHfJ3zKmdgimcQuSwUdkqZqbIq3GNn3/Ya73m9Yy63MESmQs1QxZO5qqNsi1C1xqFv8qBYUo2/MLUW5qKEVuR2p4HLERZzgVlhq9VbPzJewe3ofZPNaiucwK4PINUcsaY73EJtJWt+rAcXwjVa7hFzUUD5JyOfABU5wR5tpZNbu6JniMJ39K9Acy2eOf5SgI0rnfoRlNxdo1jVgmc0FkuWZanqDzAkOUaM6omBYq63gAie4pVBVo82dRTJkYHYwRDMSPflPYdf4uxogRpPxmRkMQi7U0GpeSZrTLT5AWQq+AMJoL2KQsTdEPlwdwDs5QNLxrxrZjT2WXttjJAd0xOn44wGoRrX9SVrxU3CBE9xKl1XsY9mNA90tHTCHj555ANqXiIs4wS3XUbUzSWSLWUlzutXf5L2IMA7EmtpkNf36nJ5ev80ypUFcNM1amUZy5ZiafJSWuxjNNgaW2VhULTFy/JjiooELnOBufNFs/y0N+ICN8SEpRQWI4RtyGl40PBZsfHOAuWsDuGiMrnu1VZTKmlucWgXaRRwUo2/xVkHdNo2LFnERZ92HiFpAJ81qt9LsaOkVRM0t1uyi2K9mxz+iVtI0tsRzr101RYa6Y45mW8DpAGjWU0DVFwXkqLbIJByvXZY3Z5F0TjBX8+CY/iNGj450owcHOXV7zu/svz+ZKOqlkwASGar/6FPOXz36/+Zvi6L8BDSI6WVE8pW5AAAAAElFTkSuQmCC);}
.DeviceIcon{
display: block;
height: 36px;
width: 36px;
left: -2px;
position: absolute;
border-radius: 3px 0 0 3px;
-moz-border-radius: 3px 0 0 3px;
-webkit-border-radius: 3px 0 0 3px;
top: 8px;
border: 1px solid silver;
border-right: none;
background-color: #F5F5F5;
background-position: 6px 5px;
background-repeat: no-repeat;
background-size: 24px auto;
background-image:url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAYAAADgdz34AAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAADsMAAA7DAcdvqGQAAAIYSURBVEhL7VO/axRBGN1/QdIIVlauOzN7K9jYCSpCmmBrqSB6szN7d5EYLMQQYmHlL2wsLSzEwGVn5m7NBTkxehKQlFpYGASxUlGLFDq+8Zawd7Bo7q4SH3zsm2+//Wbfm2+8UUGE0giTLycLny9P0VhvkVh38tTkEAkTUGG6VOhtWjXTeXp8+Ocf0bCe3UBjy2qPP5BYnc1fjQ/CWwmafqG17DtsueXLlOavxgfj6hya/0Q8IHFaydOTA41VB7Zs5svdI6iqvWjQoEm2hOciLFigQu1weP0Rz/fIXSnmS3nSXiJCX6XShL83wCyvsfqqOzgb1js2nH1iWdLe4fjwK0ZyezhfymuZdf0wae9Cd1ZhY81ix5vsgtpDRXoGf3xnXH5QqJlDlzcs4elJz+2Iw1twali1OUOkuTYKpzw9BbXXHff5ypHo0gvLYnUcFulvVLZfw4aH8LAHy96MwtH8KWKznzfdysWuZdIc86hs/UDcDePmfsqbAlNzbxReqaf73MA4HnB9Oppb7ytgyZ8tKsov8mJNEX2Lnv+9RUX5RT5U00Qv1c/v0qKi/CIfqBcrDSJa9x0fsCjEzAa4RE4alel0IMziMC9DWf0Brg9H8z1LpT7hLhosyj4RYTZwI7cg8zNyA7wsSuuleRvNPbMkSY96QZzO4qDX4ecr+NiDh+BqgJdFWT2rrb7ERrf9+eUpp+g//ml43i/diSDEnqtyKQAAAABJRU5ErkJggg==);}
.VideoIcon{
display: block;
height: 36px;
width: 36px;
left: -2px;
position: absolute;
border-radius: 3px 0 0 3px;
-moz-border-radius: 3px 0 0 3px;
-webkit-border-radius: 3px 0 0 3px;
top: 8px;
border: 1px solid silver;
border-right: none;
background-color: #F5F5F5;
background-position: 6px 5px;
background-repeat: no-repeat;
background-size: 24px auto;
background-image:url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAYAAADgdz34AAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAADsMAAA7DAcdvqGQAAAJRSURBVEhL7ZS/b9NAFMcNnRmahT8AEMK+OycSP7oCAz/Wqv8ATG18zg8nTQGJZGVmgMJSKgQSE8J3jtMEkkCLhMSE2DtRprYrLBzv+UcSxTFJNyR40ieJ7dz3+967d9b+3aBrX+bpspgMPIO/HAv/Wa8fN7h/OUSkolvySgxcPzKd7j6xxERMp7dPbf8xXRPzmpEXD81KXzGnq1j5bSpm9b3K1j4G4G9WfgdrUih3Ve7eZ0ULrRsa4fInLbUV4V4qKAiZPTWW31xACBcPmNPbNSx3F9Z/N7g8GAWqOyB264le8jOaYYlDWtwCIZkKVghi64x7OQTbxErtRVbpL56z3TNnb70+gWIxC/WvmbD/EOg4zQAyVrTYGrQLK4bywbinKPe2iN05GcklYzaDmGT7sqs7iuWb1yK5IAzunSK2twmJeSMGycXpDA1xOEhRng+Ul17N6VxcB/Fv5uo2PBe/Bgas1AknYwawXdg2NImq3wCz+6D1ghT8QIsW2wq1YZPlIWwWXHgVantXZ4HkxTPsf1iFCPYiCxkH9+zmwHhgYFa3FSu0LgVlzhCEu7VsbScymMyYwQeFmUXrpwaciTpmPEk4JlEBjNtCtH5qHLkC3ANiSWf8/ZMGVLCRvgfhlA0N4AMv8PCMvnv+RGKKLLkJOg34fpmYIsjmB16E2YzPexpxK/Cg4YCIcECW1BxZcW/C/b3hObBkB/sZADePQu7OJxgOfw/awgKDKPQV/7Rhy+dg4Gv6bT8DJnfh3dLAMmen2TCr/YZuuRcj3f/xV4am/QZUR/xNOaU7LgAAAABJRU5ErkJggg==);}
.SoundIcon{
display: block;
height: 36px;
width: 36px;
left: -2px;
position: absolute;
border-radius: 3px 0 0 3px;
-moz-border-radius: 3px 0 0 3px;
-webkit-border-radius: 3px 0 0 3px;
top: 8px;
border: 1px solid silver;
border-right: none;
background-color: #F5F5F5;
background-position: 6px 5px;
background-repeat: no-repeat;
background-size: 24px auto;
background-image:url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAYAAADgdz34AAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAADsMAAA7DAcdvqGQAAAOCSURBVEhLrVXPaxNBFA6KoCL4A9GLWEVpze7MbtIgguih1D9AlHoQREGsNjsz29Ym1VaJF0+ieFBPggcP0oLaZGd2Y62ttVYFrVoQ9FBQwV8IKla8KLK+N9kWxFoi7AdhM2++ed+8mffeJOIA5f4qwuQzIvybRBRXRub4YPJr60ymvqQ674fEKV1MFMI50VT1IC1Fiwq/hzjqUGT6A4QpZrUNhEQEvwgPdkXm6mA6pZ2wcCLd/SiEoxiJzBqZ5t7F+K0pDM03ubyePvoAovCe0JaRpZowGzLNpYXgsEBE+Tvuzu4YDglXQ9F0IsnUdqttsJ8y2aTHWbmJuOXPyDO4cjRpRhQKcwgrbzCF6qPtAyFt7UfHIHAbIlCDEQuORRbqj42FJpPvjGx5vbZx71Kq8x7YvGHbHVqSMLIyZbcNNEB4DXbubgPsoqZm76X5hMun1uGhkLpl7XwmActRtUSoCXRIHHkBbZR5W4H7g4ryryT8T4D6Gzs3Elrtt0K9G0edQCLs7httvTntfCYBBNjb7dwdHYXFbqzFDIKMGrPzd2GN14mEyYojP0x1joKA7NYLmfw0dTSzCRjNpdWQBC/tjjuhwdQBtAHnFIoa3LsKOaw+TzlKgSqod1VI1QkggHulvushcPyzODaE2h1FNR6LADjKYeaY3L+ux7y8WXO5eh+PQFbuwTQ2hbzd1Ns7N8n70sQNkDsZrwBXwwkQsLifpu4N4Mqvcd3BEV2ETHo4pq3B1soRyXexCFCmeuq74ZId7zyOoab2VC5ZjaOjr9Npmoc0/U+BWlZcazrea6wlKDrdCE3hn9ZpytQ1zICPlUIbhEJ7jOcYFdpUfcwuAJWbj1LyA1Y2Fhps7rGdB0GmjqPaRir6G6EVN6Zzo422G6xZ1dazACJ5YaFDAdnwDwHCShvA/ip1BFoFVxfRZgl/C2zyB156ksltmvgXoNlRITNwlrcwMp0RMwlweRKjhrm3dW5fHdrguKDZjQJPjlmHr67QxH/B2F9eBovPQLv+acFxRQLT7dpkwQ5oiMpskTtwnHT7NlERfMI+RHml5VQFmi3ugyp9U9+FD44ajcwaRtO5RfjNFODt4KqYPgrPJvMmDFFarQnVAu8KfpKyIB+Z/gA0yQ7syDrzmDwYmeMBdYM62P2kfmgc73Km+dG8aCoe1MI7QLl6Dhnn17aXlkdmQCLxG/4WueMLd5JFAAAAAElFTkSuQmCC);}
.MemoryIcon{
display: block;
height: 36px;
width: 36px;
left: -2px;
position: absolute;
border-radius: 3px 0 0 3px;
-moz-border-radius: 3px 0 0 3px;
-webkit-border-radius: 3px 0 0 3px;
top: 8px;
border: 1px solid silver;
border-right: none;
background-color: #F5F5F5;
background-position: 6px 5px;
background-repeat: no-repeat;
background-size: 24px auto;
background-image:url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAYAAADgdz34AAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAADsMAAA7DAcdvqGQAAAPOSURBVEhLpVbNT1RXFB9MjF2ZJhobNa01rZGZ9zUU22pNpUrTEP0HXIhErGnhvfuY4Wuim9GN0e6siwJJu7HbFph33+NjkBkElFZTqqtu22ghNSDSha58/Z377mPeTCgQPMnJPXPPOb/ffefcj4mFoljOgNFRWFSYG6jlPvtflTEi3nJ74625+rjt1Sst/LjS0v+2hCwXBE4kM9O+lhqF5n2jswidiIyhXYR/JIhrG/FB4ie7p33KNbomfZW5sxrzDkvYkigWHxPJbcO+avF/8EXfYpU9CuM/IOEm7F6yAfCNiGGeIMXYp7TkPlTN3JEEG75udN2hfEfClmSFAKsDyEKNObBHOLLZLWKMxarIBsknevttSVD0QXwLRF8kWnmSgjA/C6yHIiMqFQRL1ebPO6SrTKIEenqMVjuJvKsg+pL8+P0rsH4TwVGJEsD+G0CNCP4qYTmWarrnQpuAqP4rBIxP6Z2Fa5rlXBA4jN9fkwCrp6RXREQAlRrUn2Jcn74EhHmjo5hRbO+0wFmXwB6i5CXoBPQXClYtdzr8dNhDMkYQYq6InXRZs50mgbMegegBdpFq5apVe/Ctg2h2bXtuZ2jHLedTAg5KlMcX8PvI+05jTkrgbIiAuctGU/+b0lUmSso9qrePCwLakijRDSNTeLeBeduE33JmEoz/LoKjUkEwp5jOZ7S3460Dx9DkQ2rbUGDbzklaOREku6dQIveSyGdOi5oazcK3iDLOCtCohASq7VGJXlKZMEe9WIYu4NND+y8oLSIk6Iav0ejCScdJ1tPj5+O2c0DClqSMgPEXSJwHyXMA/Qvfs4j9uJIgzrzmZNeUr+Pa0DrGmxTmvSdhSxIS0B7Hip5gz3+E5KMqy9UZERtEp6IlAtFFOuEJkzOU94qWHsVi1iiRJFiq7X2wVbrKZLUm12by79Q1Fd4QfpPfSzB3rSbTKXXnjLSzV+8Y2VWd+ml3tZnfEdqrbVOUpkezeFrgbOSgwcadz0dBNIPSPMA4GdqY96IHDWNB7yxm4TsrcNYlkA3UsDq6CoQCKLTpDQhjNnVVhE1GM8+otvs1TrSNcjSHdsJyL7zWZRcQuIv7skHTKgUAq1/XNj8v/RsiePo+87YHHr8qGEn8qigBSkP9+lHvvNOgM6+GInCiZxDzSIRHZYUANUbD5rErroGoB/Pf4+DcCG3s9+vRJxPzvZqZ+yDeyj/WUyMn0Js/cQ4KErYkuKTkox886ES2qtJplTHi0cdcMnNXPPo0gviPeBs/JmFLsum/LYz34Ws/p78tpLo1uF9CQmKx/wAOuoAFu9YaRgAAAABJRU5ErkJggg==);}
.CDRomIcon{
display: block;
height: 36px;
width: 36px;
left: -2px;
position: absolute;
border-radius: 3px 0 0 3px;
-moz-border-radius: 3px 0 0 3px;
-webkit-border-radius: 3px 0 0 3px;
top: 8px;
border: 1px solid silver;
border-right: none;
background-color: #F5F5F5;
background-position: 6px 5px;
background-repeat: no-repeat;
background-size: 24px auto;
background-image:url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAYAAADgdz34AAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAADsMAAA7DAcdvqGQAAAPmSURBVEhLrVZNj9xEEHUixIUjX38AhYndbc8lITeunMORSBwSod213Z7ZzWYFQgwHrmGlRIgfAGIACcTY3bY3BJLsZ5AQVw4oOW1WIYdB2iwrLol5rz3ZzMdumAMllWR1d72qelVdbedI6VTHZdw7I5LifRGZwgv1XS82/Vr1XRHp0lPFBzLOz/DswGo6caPstJeYL7w4/9tf+LmS7R8r2bo2qljjHs7si9h0G3Q0jXhxNitUuRMsrlYyWalg/Ey1ji7eqkRS9qXK5wYwh8oxL9IfSlU89tvXDwUbV3/+J2RgfvdUfl5GZiNYuFHJMPuYWDXkkJwM0xmpykdM/zAwqyqvFd/Mzm9d6/uRPkt7cb73qoh1WTtJQwv6RDyVnvKU+fOoyOnUn0ctktKqD5Bg8VblRdkPjlMdRHsq/P5FNMUvsHl4MtFv2EW38+3zoOZLGowDizi3wIj6nheZb9woXUb3LPMbnN+TSVG5yiz7CysvWDCIq7J3rXOF88B26AlF3Z0oKKhgRnBuAHoO3dLykuIKzl2RKm1zDWeK5tJ6RacNRI96XADWbUuzKvZsFiwsUx4Bh1pKYmOwfxZAq81L66BlzSq/RVyseWH6NuqmZev6I9D1m1B14WkfsIWB7bixKdlqw+CDQu8Ipc/hTmw2lzbZLZuIVNVqNppLG6zBlpjtMbv7waW1Ch34FAOYxEbfmzvjncPoyTNuaasJQ3xv+TPfvTKg2WnG+cuIbpOZSKVJV9fWagiDmMSmg/6IA3CPtLn5qYizz5qgREaTFwhRzzZxGZHF59DLtKHtmIP+dA5m9YQDEZqZ2oH+TweTFKFALilC+qQBIFuvgZYBtnPivfQlWwdb7HTefRZF40WWrRV0CiMzD3AP3nHDbItFxve6CHUM4Ajn1gZFvs0ziHxbtkeDfFpk26a1d9uukdlFRCkc/SNaZc5WZEvWbbpq1fa+KtZf515k0vHoqQdt2sDMx1zfqy9a3nPn9FukQUZ60TpMioJRYlK2Zau4CsdXSYuNPNKZzX6IeyqxcMsfEtuOCgypr3mBEO0FSzLkzc6N50TYi3F59sDnNgy7iOgy1Y11Fw6268hHwakBxzfqYkcFhQ8MotgF0B+N0Jywi5RO5zj2Mhowm4NhB+DxxniidmAq88BtZ6cHKLV4UToXXLzJ2/iriNKGXUOvA2gffNcApGKMjmGVCV66pHyMLA99eI6hSz4KFm6ivfI7mJKfIM0dOe3jg3PMDjad4RE+IXid5jAR/wrQhtOAs6D181rcx3AcfWiOEv4pwLiLi7JvuUenkPMRxRr3kC0f/a+mfvQP5H/5bXGcfwEwXuFcHHTSDwAAAABJRU5ErkJggg==);}
.MonitorIcon{
display: block;
height: 36px;
width: 36px;
left: -2px;
position: absolute;
border-radius: 3px 0 0 3px;
-moz-border-radius: 3px 0 0 3px;
-webkit-border-radius: 3px 0 0 3px;
top: 8px;
border: 1px solid silver;
border-right: none;
background-color: #F5F5F5;
background-position: 6px 5px;
background-repeat: no-repeat;
background-size: 24px auto;
background-image:url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAYAAADgdz34AAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAADsMAAA7DAcdvqGQAAAHASURBVEhL7VXNLgNRFJ4FL+AtzL1zqyxEIhEtFux0Y08s9M7U0PpL1ANYWPMY5t6pKqP1kxAJHkHEWhdYX+fo3YxMCbd2/ZJvMefnO+feOXPG+ndQL5im/nGZumFn6Z+UUdsirnxOb96o9Ma1Gty+7whRCzVR26KueGV+TREu3oF1ymVkRFeeEVe8oSZqW4TLZqp0rmw3XNe3ZgxSEKVUsQFNy6YucKFooTKi/cboL4hhbDpWwPHCrPYbA65ovFvgW3QL/IjEArA/xrTfGLYXjn4pgA9iXvuNATtoIfYls5VIwR55orwyg8czIfXkHjTbZP6JLoCLaTVSbOVU4UmMWax/aqEmauM2PWDLtRc41gtWTOAz4cEjbMkY0dbyJeSAFmqitjW2c9Zj+0d9bcllhvlRjngyRrQ5S4fZxBxN1NavpT1sHkQD65fKKVRjRBv+P3TY32FzUcW7hTGOEW1wFTUd9nvg8dhqIwNd3uEL+1qgZZMPGDO0uN+r034GzcvJVOlyl3IRpLduYSoaramAUY4Tpg58GEPygcAc4oZTWqY9mBfQ9NrVHMnLnMPFBK6QbwkxhFdmMYd5VaplNCzrA5e3VO54cRPvAAAAAElFTkSuQmCC);}
.SharesIcon{
display: block;
height: 36px;
width: 36px;
left: -2px;
position: absolute;
border-radius: 3px 0 0 3px;
-moz-border-radius: 3px 0 0 3px;
-webkit-border-radius: 3px 0 0 3px;
top: 8px;
border: 1px solid silver;
border-right: none;
background-color: #F5F5F5;
background-position: 6px 5px;
background-repeat: no-repeat;
background-size: 24px auto;
background-image:url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAYAAADgdz34AAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAADsMAAA7DAcdvqGQAAANbSURBVEhL3ZVrSFNhGMf3KaKPfQi6mIVOczs7UywiP4UVFFERKEEFmpaanuM8y0uSdqOQPkT0QSjCIlGCJKfbObt7Lcus1EysqHnrYnZZEa68zafnfTfXnBMW9KUe+I+z933O//e+z/vsnez/CDbLsozhpEo2z2ZXcuJv8SY7o7HYVZyY6k398whPubFYyelLWG0DxBbeA3V+yxzFFreDkpeeKDgpMVSxRW2JMZkGOQXE5ho34CqnCEClsYAqL0A4RubUBa0hK+54O75nHVbyxgiZUmsOY7WN+xS8IWlhGZOwTMmBUngVbFwtNB2MyRbDZUpOMjGC1YFlCEkMCsvgWJWuc6xI1TmWp+gca4/Uz8+jnmI9AsQx9lgTlsMakpS5FogVbFBc1QtXbYNwxToIBy51gALHGVpWTx7xRO8fBPCGDODKQtLq9DooutkFMzPgi4HRMUgotEJ0tsGXRxfDS04ZtmdfMMC6bJGarUrTgTxLDzE5In1Wa4zQ0jvqtfbE5LQbdp1rhqijwQC89DAQQMw3FVigpPopXLjTB3vOY3egsebaY7B1j8xZPQkC2HGmKTgAP2wqrKnPHFcaL5ip0WwMf3JBd7/T+w2ga8BJx9xImpqeAdPj9xAnmOi78wGcVOsPIKvYfroJJqfcXjtPuN0zUPtgGPZfbKO72XqyEfKvd0JGeQes15ppCWc95gAYzlDJBgB2nm2Gicm5AOfYBJTV9MLGfDOsPKRDQwlL1wJbShshMlMP2PsLAHipnBXsvgnSCZuwI16+++61xhrjblzjU/T57WcXVDX3Q0PPBxj9+hOGPrrgsv4FMLkSKIKViMkRL/gDiMgh7y1rhZq2IbB1jYC2opPuqsL2GkacPynIP36MT+NOGhY4ZE4s8QCMvkmiaEyOwK1HZOhpm8qzDLDmcB1sw9o/G/zqtfbEBO5wN5YrOCBX0pDLLBBAhA1A5f89DH8LF+uee6098c01CZtP2IMDGI0xbSFAMJEzSiiyQnXzADx69QU6UKXVPRTuf9A+gEpjSlYX3qUDoYhFkXtHnm2C6BwTyFFRKM9V75dH7iJecskUQn0kw5vamTyrkxBDFd6oPs2fNzqZPJsTr6Fb9E8nPkO/RCHcX6oQzH9Xp24vooB/OGSyX0YOYfT5RN3sAAAAAElFTkSuQmCC);}
.StartUpIcon{
display: block;
height: 36px;
width: 36px;
left: -2px;
position: absolute;
border-radius: 3px 0 0 3px;
-moz-border-radius: 3px 0 0 3px;
-webkit-border-radius: 3px 0 0 3px;
top: 8px;
border: 1px solid silver;
border-right: none;
background-color: #F5F5F5;
background-position: 6px 5px;
background-repeat: no-repeat;
background-size: 24px auto;
background-image:url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAYAAADgdz34AAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAADsMAAA7DAcdvqGQAAAPFSURBVEhLrVVNbBtFFF4gCFSEQNADEq1KI9omu7O7QT1SlFvFpZVCVOitFRJOy86OnTiBC8ISPUTiViFxoAc4RFET/ox3Z9dO2jq/gkpICOqWE4ceWqm/oKpEbVK6fG88brw0iRPUTxp5PDvzvfe+996M8UjQXWgzuqttVla+w7KVosXDmiWim4yHF/WO/w+QveX0nxq1vGDYFMEhO1u54Q7NJk6+mli+/Etv2zg6/eg1W0RjLFu+0/XBXGL3T4Iw+sQSpV47G19yh2YSRFDT2zeAQvK46YUZJuRl8tLOTSTMlwk8x7ySmF5wzPJgJDdx3eah1KfWh13vFp9F2MeZiO47uUlF6gycTiAR5mU96kZILuThoD7aGmZ/+QUQjzoDZx6QURIZl3mbxx8xL7xqi1gZsPtP3Ycjw8aB5Al9fG2Q5yD72hmcShQJBmS5BwkO6y2G7Uc5ioTkcpETGC6+6kdP6c+ro7tQbQP5Vy70RgRa7zISGi6a74dv6m2GJYKDJBflxBaVBVdEPfpTGo6I9thCjjAxMWfxADUdjBEpqkWRNwZJwXg03cmDN1A9e+HxL0SuHPFL3+7O/PykplwGKuOI5cd/0iYXcjh56I3ya3ieHlE9Eh4uIMI7KuGQCNpfgVOuplwGNQwT8QISpA43E6WJmwcZQVJV0hGRKP9jcvmeplxGZz7cZnF5nipkZaIWAwYoWtT/Z8aB8f9WTvIYQixQ05AUDiRpNM+6Biqqrns0ujtT2qRJl7GrL34F3v9BBigCzGsYvyppVtS+aYBcOeYH33R4372oKdMgzbo+nE8sES8yUfnczFa6kKjJei5WIG0MGIfud7H3y/bM+HOa7mEguVM4ULO9eD/9ZyLoUYnDvfIQaWMQOV0VfnRyBc3TYJ48snOgtFnNebmjQwQMJGdVwlMSpecuySlkUZG0At0z6MwvTL/emXYubAf5fP3eqdd32oCE9lP0e1YRrAUnX3kGhyN04vy2QvVpvWx0iWAH1n+CgSXIOIHm+a25utSDwuWI3r460OZH6cHAoU/10gNYR8NO5odvO3hccC1cIN0VOUXEw1sul6/rrasD3s26g3h9hPzRHKy+RGt0C1p+ae9O/sN2tQfyUSNRTuq/8d+2CDP0rSXg0TVVEXSZcQlJ4mGszcDDE/R6IcJ9kGqJ7iaXyH15zvJkrz7eGvDuduPpcwenE5JL9YEfbXH7gpcx/536BF5fwC35sTVQ3qqPrg+UQJar3ISh6/Bu1vSl52a/f56+2V6UwdokXqzDjpZrYzCMfwFIN8vxH8HLbAAAAABJRU5ErkJggg==);}"#
                }
                body {
                    // attributes
                    div(class="HeaderHolder") {
                        div(class="HeaderTitle") {
                            table {
                                tr {
                                    td(colspan="2", class="value header Name") {
                                        : Raw("Inventory Report of localhost - Friday, March 1, 2019 3:08 PM");  //fixme
                                    }
                                }
                                tr {
                                    td(colspan="2", class="value header Group")
                                }
                            }
                        }
                        div (class="InformationType Drive") {
                            div (class="NetworkAdapterIcon"){}
                            div (class="Container") {
                                div (class="title") {
                                    : Raw("Os Version");
                                }
                                table(class="box") {
                                    /*tr {
                                        td(colspan="2", class="title2") {
                                            : "Intel(R) Dual Band Wireless-AC 3165";
                                        }
                                    }*/
                                    @ for i in 0..columns.len() {
                                        tr {
                                            @ for j in 0..map.len() {
                                                td(class="label") {
                                                    : columns[i].clone();
                                                }
                                                td(class="value") {
                                                    : map[j][i].clone();
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
    });
    println!("{}",actual);
}