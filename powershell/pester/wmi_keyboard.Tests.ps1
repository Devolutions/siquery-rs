Import-Module "$PSScriptRoot/../siquery"

Describe 'wmi_keyboard table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll {
				$SkipTableTest = (-Not $IsWindows) ? $true : $false
				$allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json"
			}

			It 'has a schema' {
				$wmi_keyboard = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_keyboard' }
				$wmi_keyboard | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$wmi_keyboard = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_keyboard' }
				$schema = Get-Content $wmi_keyboard.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			# Windows Only Table -
			It 'validates schema' -Skip:$SkipTableTest {
				$wmi_keyboard = Get-Siq 'wmi_keyboard' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'wmi_keyboard' }).FullName -Raw

				$json = $wmi_keyboard | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
