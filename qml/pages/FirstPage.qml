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

            Image {
                id: appIcon
                source: patchIcon(icon)
                width: 172
                height: 172
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
    }

    Component.onCompleted: {
        console.log(Theme.iconSizeSmall, Theme.iconSizeMedium, Theme.iconSizeLarge)
        console.log(Theme.itemSizeExtraSmall, Theme.itemSizeSmall, Theme.itemSizeMedium, Theme.itemSizeLarge, Theme.itemSizeExtraLarge)
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

    SilicaFlickable {
        anchors.fill: parent
        width: page.width

        SearchField {
            id: searchField
            width: page.width
            anchors.left: parent.left
            anchors.right: parent.right
            anchors.top: parent.top
            placeholderText: qsTr("Search")
            autoScrollEnabled: false
            inputMethodHints: Qt.ImhNoPredictiveText | Qt.ImhNoAutoUppercase
            focus: true

            onTextChanged: {
                resultModel.query = searchField.text
                searchField.focus = true
                console.log(searchField.text)
            }
        }

        SilicaListView {
            id: listView

            width: page.width
            anchors {
                top: searchField.bottom
                bottom: parent.bottom
            }
            clip: true

            model: ResultListModel {
                id: resultModel
                //query: searchFieldId.text

                onRowsAboutToBeRemoved: console.log("onRowsAboutToBeRemoved", first, last)
                onRowsAboutToBeInserted: console.log("onRowsAboutToBeInserted", start, end)
                onModelReset: console.log("onModelReset", listView.count)
            }

            delegate: resultDelegate

            onCountChanged: console.log("count", count)
        }

        // PullDownMenu and PushUpMenu must be declared in SilicaFlickable, SilicaListView or SilicaGridView
        PullDownMenu {
            MenuItem {
                text: qsTr("Show Page 2")
                onClicked: pageStack.push(Qt.resolvedUrl("SecondPage.qml"))
            }
        }
    }
}
