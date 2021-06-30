Import-Module "$PSScriptRoot/../siquery"

Describe 'wmi_bios table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll {
				$SkipTableTest = (-Not $IsWindows) ? $true : $false
				$allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json"
			}

			It 'has a schema' {
				$wmi_bios = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_bios' }
				$wmi_bios | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$wmi_bios = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_bios' }
				$schema = Get-Content $wmi_bios.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			# Windows Only Table -
			It 'validates schema' -Skip:$SkipTableTest {
				$wmi_bios = Get-Siq 'wmi_bios' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'wmi_bios' }).FullName -Raw

				$json = $wmi_bios | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
