use crate::entry::Entry;
use crate::group::Group;

// bool open(QSharedPointer<const CompositeKey> key, QString* error = nullptr);
// bool open(const QString& filePath, QSharedPointer<const CompositeKey> key, QString* error = nullptr);
// bool save(SaveAction action = Atomic, const QString& backupFilePath = QString(), QString* error = nullptr);
// bool saveAs(const QString& filePath,
// SaveAction action = Atomic,
// const QString& backupFilePath = QString(),
// QString* error = nullptr);
// bool extract(QByteArray&, QString* error = nullptr);
// bool import(const QString& xmlExportPath, QString* error = nullptr);
//
pub struct Database {
    entries: Vec<Entry>,
    groups: Vec<Group>,
}
