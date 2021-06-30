Import-Module "$PSScriptRoot/../siquery"

Describe 'wmi_motherboard table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll {
				$SkipTableTest = (-Not $IsWindows) ? $true : $false
				$allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json"
			}

			It 'has a schema' {
				$wmi_motherboard = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_motherboard' }
				$wmi_motherboard | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$wmi_motherboard = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_motherboard' }
				$schema = Get-Content $wmi_motherboard.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			# Windows Only Table -
			It 'validates schema' -Skip:$SkipTableTest {
				$wmi_motherboard = Get-Siq 'wmi_motherboard' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'wmi_motherboard' }).FullName -Raw

				$json = $wmi_motherboard | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
