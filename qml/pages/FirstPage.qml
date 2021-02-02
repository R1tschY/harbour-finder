import QtQuick 2.0
import Sailfish.Silica 1.0
import de.richardliebscher.harbour_finder 0.1

Page {
    id: page

    // The effective value will be restricted by ApplicationWindow.allowedOrientations
    allowedOrientations: Orientation.All

    Component {
        id: resultDelegate

        BackgroundItem {
            width: page.width
            implicitHeight: Theme.itemSizeMedium

            Label {
                text: index
            }
        }
    }

    Process {
        id: lca_tool
        program: "lca-tool"

        function open(file) {
            lca_tool.arguments = ["--triggerdesktop", file]
            lca_tool.start()
        }
        // TODO: Check exit code:
        // 0   success
        // 1   not enough arguments
        // 2   problems with the arguments
        // 3   triggered an action not applicable to the given URIS
        // 4   no default action exists for the given URIS
        // 5   desktop file not found

        onRunningChanged: console.log("running: " + lca_tool.running)
    }

    function patchIcon(icon) {
        if (!icon)
            return ""

        if (icon[0] === "/" || icon.indexOf("image:") === 0)
            return icon

        return "image://theme/" + icon
    }

    // To enable PullDownMenu, place our content in a SilicaFlickable
    SilicaListView {
        anchors.fill: parent

        // PullDownMenu and PushUpMenu must be declared in SilicaFlickable, SilicaListView or SilicaGridView
        PullDownMenu {
            MenuItem {
                text: qsTr("Show Page 2")
                onClicked: pageStack.push(Qt.resolvedUrl("SecondPage.qml"))
            }
        }

        header: PageHeader {
            title: qsTr("Finder")
        }

        model: ResultListModel {
            id: resultModel
        }

        delegate: BackgroundItem {
            width: page.width
            implicitHeight: Theme.itemSizeMedium

            Image {
                id: appIcon
                source: patchIcon(icon)
                width: Theme.itemSizeMedium
                height: Theme.itemSizeMedium
            }

            Label {
                id: appNameLabel
                anchors {
                    left: appIcon.right
                    leftMargin: Theme.paddingMedium
                }

                text: name

            }

            onClicked: {
                if (fileName) {
                    console.log(fileName)
                    lca_tool.open(fileName)
                } else {
                    console.error("no valid file name for desktop file")
                }
            }
        }

        onCountChanged: console.log("count", count)
    }
}
