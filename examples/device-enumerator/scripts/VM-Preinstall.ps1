# Installa Python
winget install Python.Python.3.12 `
  --accept-source-agreements `
  --accept-package-agreements

# Ricarica PATH (potrebbe richiedere una nuova shell)
$env:Path += ";$env:LocalAppData\Programs\Python\Python312"
$env:Path += ";$env:LocalAppData\Programs\Python\Python312\Scripts"

# Aggiorna pip
python -m pip install --upgrade pip

# Installa PyViGEm
python -m pip install pyvigem

Write-Host ""
Write-Host "====================================="
Write-Host "Python e PyViGEm installati"
Write-Host "====================================="